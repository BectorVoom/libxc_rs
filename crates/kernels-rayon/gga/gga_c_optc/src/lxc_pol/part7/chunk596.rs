//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 596/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk596(t1038: f64, t2950: f64, t2843: f64, t373: f64, t2942: f64, t1045: f64, t2865: f64, t2845: f64, t2852: f64, t2858: f64, t2862: f64, t2867: f64, t2871: f64, t2874: f64, t2877: f64, t2943: f64) -> (f64, f64, f64, f64, f64) {
    let t2951 = t1038 * t2950;
    let t2953 = 0.68863333333333333333e0_f64 * t2843;
    let t2958 = 1.0_f64/f64::sqrt(t373);
    let t2959 = t2958 * t2942;
    let t2961 = t1045 * t2950;
    let t2963 = 0.17365833333333333333e0_f64 * t2865;
    let t2968 = -0.17648625e1_f64 * t2943 + 0.3529725e1_f64 * t2951 + t2953 + 0.34431666666666666666e0_f64 * t2845 - 0.34431666666666666667e0_f64 * t2852 + 0.103295e1_f64 * t2858 - 0.516475e0_f64 * t2862 + 0.31558125e0_f64 * t2959 + 0.6311625e0_f64 * t2961 + t2963 + 0.13892666666666666667e0_f64 * t2867 - 0.34731666666666666667e-1_f64 * t2871 + 0.20839e0_f64 * t2874 - 0.104195e0_f64 * t2877;
    (t2951, t2958, t2959, t2961, t2968)
}
