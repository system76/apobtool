use plain::Plain;
use std::{
    fmt,
    fs,
    slice,
};

#[derive(Debug)]
#[repr(C)]
pub struct ApobBaseHeader {
    pub signature: u32,
    pub version: u32,
    pub size: u32,
    pub offset_of_first_entry: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct ApobHeader {
    pub header: ApobBaseHeader,
    pub sys_map_offset: u32,
    pub mem_smbios_offset: u32,
    pub nvdimm_info_offset: u32,
    pub apob_apcb_boot_info_offset: u32,
    pub sys_nps_offset: u32,
    pub reserved: [u32; 2],
    //TODO
}
unsafe impl Plain for ApobHeader {}

#[derive(Debug)]
#[repr(C)]
pub struct ApobHmac {
    apob_hmac: [u8; 32],
}

#[derive(Debug)]
#[repr(C)]
pub struct ApobTypeHeader {
    pub group_id: u32,
    pub data_type_id: u32,
    pub instance_id: u32,
    pub type_size: u32,
    pub apob_type_hmac: ApobHmac,
}

#[derive(Debug)]
#[repr(C)]
pub enum MemoryHoleTypes {
    UMA,
    MMIO,
    PrivilegedDRAM,
    Reserved1TbRemap,
    MaxMemoryHoleTypes,
}

#[derive(Debug)]
#[repr(C)]
pub struct MemoryHoleDescriptor {
    pub base: u64,
    pub size: u64,
    pub kind: MemoryHoleTypes,
}

#[repr(C)]
pub struct SystemMemoryMap {
    pub top_of_system_memory: u64,
    pub number_of_holes: u32,
    // pub hole_info: [MemoryHoleDescriptor; number_of_holes],
}

impl SystemMemoryMap {
    pub unsafe fn hole_info(&self) -> &[MemoryHoleDescriptor] {
        slice::from_raw_parts(
            (self as *const Self).add(1) as *const MemoryHoleDescriptor,
            self.number_of_holes as usize
        )
    }
}

impl fmt::Debug for SystemMemoryMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SystemMemoryMap")
         .field("top_of_system_memory", &self.top_of_system_memory)
         .field("number_of_holes", &self.number_of_holes)
         .field("hole_info", &unsafe { self.hole_info() })
         .finish()
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct ApobSystemMemoryMapType {
    pub apob_type_header: ApobTypeHeader,
    pub apob_system_map: SystemMemoryMap,
}
unsafe impl Plain for ApobSystemMemoryMapType {}

fn main() {
    let data = fs::read("apob.rom").expect("failed to read apob.rom");
    println!("ROM size: {}", data.len());

    if &data[0..4] != b"APOB" {
        panic!("signature not found");
    }

    let header: &ApobHeader = plain::from_bytes(&data).expect("failed to cast ApobHeader");
    println!("{:#X?}", header);

    let mem: &ApobSystemMemoryMapType = plain::from_bytes(&data[header.sys_map_offset as usize..]).expect("failed to cast ApobSystemMemoryMapType");
    println!("{:#X?}", mem);
}
