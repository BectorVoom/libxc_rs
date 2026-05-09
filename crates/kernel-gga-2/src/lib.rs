#![allow(clippy::excessive_precision)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::too_many_arguments)]

//! GGA kernel translations batch 2.

pub mod gga_x_vmt;

// Deferred: CubeCL proc macro SIGSEGV on large lxc_pol/kxc_pol files.
// pub mod gga_c_gaploc;
// pub mod gga_x_hjs_b88_v2;
// pub mod gga_x_wpbeh;
