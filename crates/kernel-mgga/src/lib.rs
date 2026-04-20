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
pub use libxc_kernel_mgga_1a as batch1a;
pub use libxc_kernel_mgga_1b as batch1b;
pub use libxc_kernel_mgga_1c as batch1c;
pub use libxc_kernel_mgga_1d as batch1d;
pub use libxc_kernel_mgga_2a as batch2a;
pub use libxc_kernel_mgga_2b as batch2b;
pub use libxc_kernel_mgga_3a as batch3a;
pub use libxc_kernel_mgga_3b as batch3b;
pub use libxc_kernel_mgga_4a as batch4a;
pub use libxc_kernel_mgga_4b as batch4b;
pub use libxc_kernel_mgga_5 as batch5;
pub use libxc_kernel_mgga_6 as batch6;
pub use libxc_kernel_mgga_7a as batch7a;
pub use libxc_kernel_mgga_7b as batch7b;
pub use libxc_kernel_mgga_7c as batch7c;
pub use libxc_kernel_mgga_7d as batch7d;
pub use libxc_kernel_mgga_7e as batch7e;
pub use libxc_kernel_mgga_8a as batch8a;
pub use libxc_kernel_mgga_8b as batch8b;
pub use libxc_kernel_mgga_8c as batch8c;
pub use libxc_kernel_mgga_8d as batch8d;
pub use libxc_kernel_mgga_8e as batch8e;
pub use libxc_kernel_mgga_8f as batch8f;
pub use libxc_kernel_mgga_9a as batch9a;
pub use libxc_kernel_mgga_9b as batch9b;
pub use libxc_kernel_mgga_10a as batch10a;
pub use libxc_kernel_mgga_10b as batch10b;
pub use libxc_kernel_mgga_11a as batch11a;
pub use libxc_kernel_mgga_11b as batch11b;
pub use libxc_kernel_mgga_11c as batch11c;
pub use libxc_kernel_mgga_12a as batch12a;
pub use libxc_kernel_mgga_12b as batch12b;
pub use libxc_kernel_mgga_12c as batch12c;
pub use libxc_kernel_mgga_12d as batch12d;
pub use libxc_kernel_mgga_13a as batch13a;
pub use libxc_kernel_mgga_13b as batch13b;
pub use libxc_kernel_mgga_14a as batch14a;
pub use libxc_kernel_mgga_14b as batch14b;
pub use libxc_kernel_mgga_14c as batch14c;
pub use libxc_kernel_mgga_14d as batch14d;
pub use libxc_kernel_mgga_14e as batch14e;
pub use libxc_kernel_mgga_14f as batch14f;
pub use libxc_kernel_mgga_14g as batch14g;
pub use libxc_kernel_mgga_14h as batch14h;
pub use libxc_kernel_mgga_14i as batch14i;
pub use libxc_kernel_mgga_14j as batch14j;
pub use libxc_kernel_mgga_14k as batch14k;
pub use libxc_kernel_mgga_14l as batch14l;
pub use libxc_kernel_mgga_15a as batch15a;
pub use libxc_kernel_mgga_15b as batch15b;
pub use libxc_kernel_mgga_15c as batch15c;
pub use libxc_kernel_mgga_15d as batch15d;
pub use libxc_kernel_mgga_15e as batch15e;
pub use libxc_kernel_mgga_15f as batch15f;
pub use libxc_kernel_mgga_15g as batch15g;
pub use libxc_kernel_mgga_15h as batch15h;
pub use libxc_kernel_mgga_16 as batch16;
pub use libxc_kernel_mgga_17 as batch17;
pub use libxc_kernel_mgga_18a as batch18a;
pub use libxc_kernel_mgga_18b as batch18b;
pub use libxc_kernel_mgga_18c as batch18c;
pub use libxc_kernel_mgga_19a as batch19a;
pub use libxc_kernel_mgga_19b as batch19b;
pub use libxc_kernel_mgga_19c as batch19c;
pub use libxc_kernel_mgga_19d as batch19d;
pub use libxc_kernel_mgga_19e as batch19e;
pub use libxc_kernel_mgga_19f as batch19f;
pub use libxc_kernel_mgga_19g as batch19g;
pub use libxc_kernel_mgga_20a as batch20a;
pub use libxc_kernel_mgga_20b as batch20b;
pub use libxc_kernel_mgga_20c as batch20c;
pub use libxc_kernel_mgga_20d as batch20d;
pub use libxc_kernel_mgga_20e as batch20e;
pub use libxc_kernel_mgga_20f as batch20f;
pub use libxc_kernel_mgga_20g as batch20g;
pub use libxc_kernel_mgga_20h as batch20h;
pub use libxc_kernel_mgga_20i as batch20i;
pub use libxc_kernel_mgga_20j as batch20j;
pub use libxc_kernel_mgga_20k as batch20k;
pub use libxc_kernel_mgga_20l as batch20l;
pub use libxc_kernel_mgga_21 as batch21;
pub use libxc_kernel_mgga_22a as batch22a;
pub use libxc_kernel_mgga_22b as batch22b;
pub use libxc_kernel_mgga_22c as batch22c;
pub use libxc_kernel_mgga_22d as batch22d;
pub use libxc_kernel_mgga_23 as batch23;
pub use libxc_kernel_mgga_24a as batch24a;
pub use libxc_kernel_mgga_24b as batch24b;
pub use libxc_kernel_mgga_25a as batch25a;
pub use libxc_kernel_mgga_25b as batch25b;
pub use libxc_kernel_mgga_26a as batch26a;
pub use libxc_kernel_mgga_26b as batch26b;
pub use libxc_kernel_mgga_27a as batch27a;
pub use libxc_kernel_mgga_27b as batch27b;
pub use libxc_kernel_mgga_27c as batch27c;
pub use libxc_kernel_mgga_28 as batch28;
pub use libxc_kernel_mgga_29 as batch29;
pub use libxc_kernel_mgga_30 as batch30;
pub use libxc_kernel_mgga_31 as batch31;
pub use libxc_kernel_mgga_32a as batch32a;
pub use libxc_kernel_mgga_32b as batch32b;
pub use libxc_kernel_mgga_33 as batch33;
pub use libxc_kernel_mgga_34 as batch34;
pub use libxc_kernel_mgga_35 as batch35;
pub use libxc_kernel_mgga_36a as batch36a;
pub use libxc_kernel_mgga_36b as batch36b;
pub use libxc_kernel_mgga_37a as batch37a;
pub use libxc_kernel_mgga_37b as batch37b;

pub mod deferred;
