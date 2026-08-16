//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 483/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk483(t2925: f64, t2846: f64, t941: f64, t945: f64, t307: f64, t944: f64, t302: f64, t2904: f64, t310: f64, t960: f64, t964: f64, t320: f64, t963: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2926 = 1.0_f64 / t2925;
    let t2930 = 0.22831111111111111111e-1_f64 * t2846;
    let t2938 = t941 * t945;
    let t2941 = t944 * t307;
    let t2942 = 1.0_f64 / t2941;
    let t2943 = t302 * t2942;
    let t2950 = 0.68863333333333333333e0_f64 * t2846;
    let t2957 = 0.17365833333333333333e0_f64 * t2904;
    let t2966 = t944 * t944;
    let t2967 = 1.0_f64 / t2966;
    let t2968 = t302 * t2967;
    let t2969 = t310 * t310;
    let t2970 = 1.0_f64 / t2969;
    let t2974 = 0.12361111111111111111e-1_f64 * t2846;
    let t2982 = t960 * t964;
    let t2985 = t963 * t320;
    (t2926, t2930, t2938, t2943, t2950, t2957, t2968, t2970, t2974, t2982, t2985)
}
