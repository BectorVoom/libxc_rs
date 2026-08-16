//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1168/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1168(t23576: f64, t23581: f64, t23583: f64, t23585: f64, t23587: f64, t23592: f64, t23597: f64, t23602: f64, t23605: f64, t23608: f64, t23612: f64, t23614: f64, t23616: f64, t23620: f64) -> f64 {
    let t24263 = -0.85199506172839506175e-1_f64 * t23576 - 0.82156666666666666667e-1_f64 * t23581 - 0.54771111111111111111e0_f64 * t23583 + 0.21908444444444444444e0_f64 * t23585 - 0.13145066666666666666e1_f64 * t23587 + 0.98587999999999999999e0_f64 * t23592 - 0.10954222222222222222e0_f64 * t23597 - 0.21908444444444444444e0_f64 * t23602 - 0.71752000000000000002e1_f64 * t23605 - 0.59793333333333333333e0_f64 * t23608 + 0.71752e1_f64 * t23612 + 0.15944888888888888889e1_f64 * t23614 + 0.23917333333333333333e1_f64 * t23616 - 0.79724444444444444446e0_f64 * t23620;
    t24263
}
