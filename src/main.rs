use std::sync::Arc;

use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::device::{Device, QueuesIter};
use vulkano::device::Features;
use vulkano::device::physical::PhysicalDevice;
use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;
use vulkano::Version;

struct System {
    vk_instance: Arc<Instance>,
}

impl System {
    fn print_devices(&self) {
        for physical_device in PhysicalDevice::enumerate(&self.vk_instance) {
            println!("Available device: {}", physical_device.properties().device_name);
            for family in physical_device.queue_families() {
                println!("{}: a queue family with id {} ", physical_device.properties().device_name, family.id());
            }
        }
    }
}

impl Default for System {
    fn default() -> Self {
        let instance = match Instance::new(None, Version::V1_1, &InstanceExtensions::none(), None) {
            Ok(i) => i,
            Err(err) => panic!("Couldn't build instance: {:?}", err)
        };
        return System {
            vk_instance: instance
        };
    }
}

fn main() {
    let system = System { ..Default::default() };
    system.print_devices();
}
