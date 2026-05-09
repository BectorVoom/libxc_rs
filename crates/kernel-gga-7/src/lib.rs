#![allow(clippy::excessive_precision)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::too_many_arguments)]

//! GGA kernel translations batch 7.

pub mod gga_c_lypr;
pub mod gga_c_op_b88;
pub mod gga_c_op_g96;
pub mod gga_c_op_pbe;
pub mod gga_c_op_pw91;
pub mod gga_c_p86vwn;
pub mod gga_c_pbe;
pub mod gga_c_pbe_vwn;
pub mod gga_c_scan_e0;
pub mod gga_k_thakkar;
pub mod gga_x_am05;
pub mod gga_x_chachiyo;
pub mod gga_x_gg99;
pub mod gga_x_ityh;
pub mod gga_x_ityh_optx;
pub mod gga_x_ityh_pbe;
pub mod gga_x_n12;
pub mod gga_x_ncap;
pub mod gga_x_sfat;
pub mod gga_x_sfat_pbe;
pub mod hyb_gga_x_cam_s12;
pub mod hyb_gga_xc_case21;

// Deferred: CubeCL proc macro SIGSEGV on large lxc_pol/kxc_pol files.
// pub mod gga_c_pbeloc;
// pub mod gga_c_pw91;
// pub mod gga_c_regtpss;
// pub mod gga_c_revtca;
// pub mod gga_c_sogga11;
// pub mod gga_c_zpbeint;
// pub mod gga_c_zvpbeloc;
// pub mod gga_xc_b97;
