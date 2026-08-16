//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 588/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk588(t2843: f64, t2865: f64, t2845: f64, t2852: f64, t2858: f64, t2862: f64, t2867: f64, t2871: f64, t2874: f64, t2877: f64) -> f64 {
    let t2890 = 0.96922222222222222222e3_f64 * t2843;
    let t2895 = 0.13111111111111111111e3_f64 * t2865;
    let t2900 = t2890 + 0.48461111111111111112e3_f64 * t2845 - 0.48461111111111111111e3_f64 * t2852 + 0.14538333333333333333e4_f64 * t2858 - 0.72691666666666666667e3_f64 * t2862 + t2895 + 0.10488888888888888889e3_f64 * t2867 - 0.26222222222222222222e2_f64 * t2871 + 0.15733333333333333333e3_f64 * t2874 - 0.78666666666666666667e2_f64 * t2877;
    t2900
}
