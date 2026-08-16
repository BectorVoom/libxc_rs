//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta52 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk366;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk367;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk368;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk369;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk370;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk371;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk372;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk373;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta52(t1036: f64, t370: f64, t368: f64, t372: f64, t364: f64, t354: f64, t270: f64, t283: f64, t61: f64, t248: f64, t884: f64, t1000: f64, t1005: f64, t1020: f64, t1025: f64, t1032: f64, t350: f64, t378: f64, t964: f64, t973: f64, t997: f64, t349: f64, t225: f64, t382: f64, t386: f64, t68: f64, t1011: f64, t1014: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1038, t1040) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk366(t1036, t370, t368, t372, t364);
        let t1041 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk367(t1040, t354);
        let t1043 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk368(t270, t283);
        let t1044 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk369(t1043, t61);
        let (t1046, t1049) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk370(t1044, t248, t884, t1000, t1005, t1020, t1025, t1032, t1038, t1041, t350, t378, t964, t973, t997);
        let (t1050, t1052) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk371(t1049, t349, t225, t382);
        let (t1053, t1055) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk372(t386, t68);
        let t1057 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk373(t1011, t1014);
    (t1038, t1040, t1041, t1043, t1044, t1046, t1049, t1050, t1052, t1053, t1055, t1057)
}
