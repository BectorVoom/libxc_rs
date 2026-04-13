#![allow(clippy::excessive_precision)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::too_many_arguments)]

//! MGGA kernel translations from maple2c.
//!
//! 92 MGGA functionals total across 37 sub-crates.
//! Each sub-crate is sized via first-fit-decreasing bin packing to stay under
//! ~50K lines of generated Rust, avoiding OOM during CubeCL proc macro expansion.
//! Seven large functionals (62K-86K lines) occupy solo crates since they exceed
//! 50K even as single modules.

// Re-export sub-crates containing compiled MGGA functionals.
pub use libxc_kernel_mgga_1 as batch1;
pub use libxc_kernel_mgga_2 as batch2;
pub use libxc_kernel_mgga_3 as batch3;
pub use libxc_kernel_mgga_4 as batch4;
pub use libxc_kernel_mgga_5 as batch5;
pub use libxc_kernel_mgga_6 as batch6;
pub use libxc_kernel_mgga_7 as batch7;
pub use libxc_kernel_mgga_8 as batch8;
pub use libxc_kernel_mgga_9 as batch9;
pub use libxc_kernel_mgga_10 as batch10;
pub use libxc_kernel_mgga_11 as batch11;
pub use libxc_kernel_mgga_12 as batch12;
pub use libxc_kernel_mgga_13 as batch13;
pub use libxc_kernel_mgga_14 as batch14;
pub use libxc_kernel_mgga_15 as batch15;
pub use libxc_kernel_mgga_16 as batch16;
pub use libxc_kernel_mgga_17 as batch17;
pub use libxc_kernel_mgga_18 as batch18;
pub use libxc_kernel_mgga_19 as batch19;
pub use libxc_kernel_mgga_20 as batch20;
pub use libxc_kernel_mgga_21 as batch21;
pub use libxc_kernel_mgga_22 as batch22;
pub use libxc_kernel_mgga_23 as batch23;
pub use libxc_kernel_mgga_24 as batch24;
pub use libxc_kernel_mgga_25 as batch25;
pub use libxc_kernel_mgga_26 as batch26;
pub use libxc_kernel_mgga_27 as batch27;
pub use libxc_kernel_mgga_28 as batch28;
pub use libxc_kernel_mgga_29 as batch29;
pub use libxc_kernel_mgga_30 as batch30;
pub use libxc_kernel_mgga_31 as batch31;
pub use libxc_kernel_mgga_32 as batch32;
pub use libxc_kernel_mgga_33 as batch33;
pub use libxc_kernel_mgga_34 as batch34;
pub use libxc_kernel_mgga_35 as batch35;
pub use libxc_kernel_mgga_36 as batch36;
pub use libxc_kernel_mgga_37 as batch37;
