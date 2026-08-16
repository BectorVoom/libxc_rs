//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta125 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk842;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk843;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk844;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk845;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk846;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk847;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk848;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta125(t1021: f64, t248: f64, t3041: f64, t1030: f64, t372: f64, t364: f64, t354: f64, t1043: f64, t121: f64, t884: f64, t1041: f64, t1044: f64, t2780: f64, t283: f64, t883: f64, t61: f64, t2771: f64, t363: f64, t368: f64, t1017: f64, t67: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3043, t3047) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk842(t1021, t248, t3041, t1030, t372, t364);
        let t3048 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk843(t3047, t354);
        let t3051 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk844(t1043, t121);
        let t3053 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk845(t248, t3051, t884);
        let (t3054, t3057, t3061) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk846(t1041, t3053, t1044, t248, t2780, t283, t883);
        let (t3062, t3064, t3067) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk847(t3061, t61, t248, t2771, t363, t368);
        let t3068 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk848(t1017, t67);
    (t3043, t3047, t3048, t3051, t3053, t3054, t3057, t3061, t3062, t3064, t3067, t3068)
}
