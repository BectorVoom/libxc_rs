//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta53 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk384;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk385;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk386;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk387;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk388;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk389;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk390;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk391;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk392;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk393;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta53(t270: f64, t283: f64, t61: f64, t248: f64, t884: f64, t1000: f64, t1005: f64, t1020: f64, t1025: f64, t1032: f64, t1038: f64, t1041: f64, t350: f64, t378: f64, t964: f64, t973: f64, t997: f64, t349: f64, t225: f64, t382: f64, t386: f64, t68: f64, t1011: f64, t1014: f64, t1010: f64, t1022: f64, t381: f64, t357: f64, t360: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1043 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk384(t270, t283);
        let t1044 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk385(t1043, t61);
        let t1046 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk386(t1044, t248, t884);
        let t1049 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk387(t1000, t1005, t1020, t1025, t1032, t1038, t1041, t1046, t350, t378, t964, t973, t997);
        let (t1050, t1052) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk388(t1049, t349, t225, t382);
        let (t1053, t1055) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk389(t386, t68);
        let t1057 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk390(t1011, t1014);
        let t1058 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk391(t1010, t1057);
        let (t1059, t1060) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk392(t1022, t381, t357, t360);
        let t1061 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk393(t1059, t1060);
    (t1043, t1044, t1046, t1049, t1050, t1052, t1053, t1055, t1057, t1058, t1060, t1061)
}
