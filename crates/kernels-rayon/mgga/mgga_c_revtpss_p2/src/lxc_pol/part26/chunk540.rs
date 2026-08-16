//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 540/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk540(t225: f64, t3259: f64, t385: f64, t1071: f64, t342: f64, t1077: f64, t384: f64, t1096: f64, t1086: f64, t989: f64, t1082: f64, t3059: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3261 = t3259 * t225 * t385;
    let t3264 = t342 * t1071;
    let t3268 = 1.0_f64 / t1077 / t384;
    let t3269 = t225 * t3268;
    let t3270 = t1096 * t1096;
    let t3271 = t3269 * t3270;
    let t3278 = t989 * t1086;
    let t3283 = t1082 * t3059;
    (t3261, t3264, t3269, t3270, t3271, t3278, t3283)
}
