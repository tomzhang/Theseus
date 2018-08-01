#![no_std]
#![feature(alloc)]

#![allow(dead_code)]

extern crate alloc;


pub struct UsbEndpDesc
{
    len: u8,
    endp_type: u8,
    addr: u8,
    attributes: u8,
    maxpacketsize: u16,
    interval: u8,
}

pub struct UsbDevice{

    port: u32,
    speed: u32,
    addr: u32,
    maxpacketsize: u32,
    endpoint: UsbEndpDesc,
}