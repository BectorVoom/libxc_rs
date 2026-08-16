//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1345/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1345(t26309: f64, t26311: f64, t26314: f64, t26319: f64, t26324: f64, t26326: f64, t26328: f64, t26330: f64, t26332: f64, t26339: f64, t26343: f64, t26836: f64) -> f64 {
    let t26846 = 0.47488888888888888888e-1_f64 * t26309 - 0.94977777777777777776e-1_f64 * t26311 + t26836 + 0.23744444444444444444e-1_f64 * t26314 + 0.14246666666666666667e0_f64 * t26319 - 0.47488888888888888888e-1_f64 * t26324 - 0.47488888888888888888e-1_f64 * t26326 - 0.31659259259259259258e-1_f64 * t26328 + 0.94977777777777777776e-1_f64 * t26330 + 0.73871604938271604937e-1_f64 * t26332 - 0.52765432098765432099e-1_f64 * t26339 - 0.17808333333333333333e-1_f64 * t26343;
    t26846
}
