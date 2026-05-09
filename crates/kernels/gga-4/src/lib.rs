#![allow(clippy::excessive_precision)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::too_many_arguments)]

//! GGA kernel translations batch 4.

pub mod gga_c_bmk;
pub mod gga_c_pbe;
pub mod gga_c_pbe_vwn;
pub mod gga_c_scan_e0;
pub mod gga_x_gg99;
pub mod gga_x_hcth_a;
pub mod gga_x_ityh_pbe;
pub mod gga_x_sfat;
pub mod gga_x_sfat_pbe;
pub mod hyb_gga_x_cam_s12;

// Deferred: CubeCL proc macro SIGSEGV on large lxc_pol/kxc_pol files.
// pub mod gga_c_hcth_a;
// pub mod gga_c_optc;
// pub mod gga_c_pbeloc;
// pub mod gga_c_pw91;
// pub mod gga_c_regtpss;
// pub mod gga_c_revtca;
// pub mod gga_c_sg4;
// pub mod gga_c_sogga11;
// pub mod gga_c_zpbeint;
// pub mod gga_c_zvpbeint;
// pub mod gga_c_zvpbeloc;
// pub mod gga_x_ft97;
// pub mod gga_x_lcgau;
// pub mod gga_xc_b97;
// pub mod hyb_gga_xc_wb97;
