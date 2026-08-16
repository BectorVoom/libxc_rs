//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 505/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk505(t2962: f64, t954: f64, t944: f64, t302: f64, t310: f64, t2944: f64, t2846: f64, t2848: f64, t2855: f64, t2860: f64, t2864: f64, t324: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2963 = t2962 * t954;
    let t2966 = t944 * t944;
    let t2967 = 1.0_f64 / t2966;
    let t2968 = t302 * t2967;
    let t2969 = t310 * t310;
    let t2970 = 1.0_f64 / t2969;
    let t2971 = t2944 * t2970;
    let t2974 = 0.12361111111111111111e-1_f64 * t2846;
    let t2979 = t2974 + 0.61805555555555555556e-2_f64 * t2848 - 0.61805555555555555555e-2_f64 * t2855 + 0.18541666666666666667e-1_f64 * t2860 - 0.92708333333333333333e-2_f64 * t2864;
    let t2980 = t2979 * t324;
    (t2963, t2966, t2967, t2968, t2969, t2970, t2971, t2979, t2980)
}
