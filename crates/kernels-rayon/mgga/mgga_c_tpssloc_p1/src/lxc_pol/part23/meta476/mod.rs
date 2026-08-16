//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta476 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1427;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1428;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta476(t449: f64, t78211: f64, t78223: f64, t300: f64, t14850: f64, t21724: f64, t1118: f64, t11190: f64, t78129: f64, t6020: f64, t3264: f64, t3313: f64, t3315: f64, t78118: f64, t78120: f64, t78122: f64, t78125: f64, t78128: f64, t78132: f64, t78196: f64, t78199: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t78225, t78227, t78229, t78232, t78236, t78239) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1427(t449, t78211, t78223, t300, t14850, t21724, t1118, t11190, t78129, t6020, t3264, t3313, t3315);
        let t78240 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1428(t78118, t78120, t78122, t78125, t78128, t78132, t78196, t78199, t78227, t78229, t78232, t78236, t78239);
    (t78225, t78227, t78229, t78232, t78236, t78239, t78240)
}
