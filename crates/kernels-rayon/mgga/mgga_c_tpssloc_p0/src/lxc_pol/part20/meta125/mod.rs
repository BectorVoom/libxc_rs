//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta125 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk814;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk815;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk816;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk817;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk818;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk819;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk820;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk821;
use chunk8::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk822;
use chunk9::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk823;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta125(t3036: f64, t368: f64, t1015: f64, t3033: f64, t1022: f64, t360: f64, t1021: f64, t248: f64, t1030: f64, t372: f64, t364: f64, t354: f64, t1043: f64, t121: f64, t884: f64, t1041: f64, t1044: f64, t2780: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3037, t3038) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk814(t3036, t368, t1015);
        let t3039 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk815(t3033, t3038);
        let t3040 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk816(t1022);
        let t3041 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk817(t3040, t360);
        let t3043 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk818(t1021, t248, t3041);
        let t3047 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk819(t1030, t372, t364);
        let t3048 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk820(t3047, t354);
        let t3051 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk821(t1043, t121);
        let t3053 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk822(t248, t3051, t884);
        let (t3054, t3057) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk823(t1041, t3053, t1044, t248, t2780);
    (t3037, t3038, t3039, t3040, t3041, t3043, t3047, t3048, t3051, t3053, t3054, t3057)
}
