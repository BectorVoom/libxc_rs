//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1341/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1341(t26261: f64, t26309: f64, t26311: f64, t26314: f64, t26319: f64, t26324: f64, t26326: f64, t26328: f64, t26330: f64, t26332: f64, t26339: f64, t26343: f64) -> f64 {
    let t26780 = 0.96141975308641975307e-1_f64 * t26261;
    let t26790 = 0.24722222222222222222e-1_f64 * t26309 - 0.49444444444444444444e-1_f64 * t26311 + t26780 + 0.12361111111111111111e-1_f64 * t26314 + 0.74166666666666666668e-1_f64 * t26319 - 0.24722222222222222222e-1_f64 * t26324 - 0.24722222222222222222e-1_f64 * t26326 - 0.16481481481481481482e-1_f64 * t26328 + 0.49444444444444444445e-1_f64 * t26330 + 0.38456790123456790123e-1_f64 * t26332 - 0.27469135802469135803e-1_f64 * t26339 - 0.92708333333333333333e-2_f64 * t26343;
    t26790
}
