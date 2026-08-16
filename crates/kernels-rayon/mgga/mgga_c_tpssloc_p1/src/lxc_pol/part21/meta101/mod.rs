//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta101 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk700;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk701;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk702;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk703;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk704;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk705;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk706;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk707;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta101(t157: f64, t2516: f64, t153: f64, t145: f64, t2447: f64, t185: f64, t193: f64, t2373: f64, t2377: f64, t2378: f64, t2379: f64, t2408: f64, t2417: f64, t2423: f64, t2426: f64, t2429: f64, t2432: f64, t2450: f64, t201: f64, t868: f64, t870: f64, t2369: f64, t2509: f64, t2512: f64, t761: f64, t172: f64, t753: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t2517 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk700(t157, t2516);
        let t2518 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk701(t153, t2517);
        let (t2519, t2520, t2521) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk702(t145, t2447, t185, t193, t2373, t2377, t2378, t2379, t2408, t2417, t2423, t2426, t2429, t2432, t2450, t2518);
        let t2522 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk703(t193, t201);
        let t2523 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk704(t868, t870);
        let (t2527, t2528) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk705(t2369, t2509, t2512);
        let t2530 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk706(t2528, t761);
        let t2531 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk707(t172, t753);
    (t2517, t2518, t2519, t2520, t2521, t2522, t2523, t2527, t2528, t2530, t2531)
}
