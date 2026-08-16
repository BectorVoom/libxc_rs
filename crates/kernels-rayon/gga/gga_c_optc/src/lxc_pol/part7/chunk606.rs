//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 606/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk606(t2994: f64, t3020: f64, t3018: f64, t2843: f64, t2845: f64, t2852: f64, t2858: f64, t2862: f64) -> (f64, f64, f64) {
    let t3021 = t2994 * t3020;
    let t3023 = 0.16081824322151104822e2_f64 * t3018 * t3021;
    let t3024 = 0.12361111111111111111e-1_f64 * t2843;
    let t3029 = t3024 + 0.61805555555555555556e-2_f64 * t2845 - 0.61805555555555555555e-2_f64 * t2852 + 0.18541666666666666667e-1_f64 * t2858 - 0.92708333333333333333e-2_f64 * t2862;
    (t3021, t3023, t3029)
}
