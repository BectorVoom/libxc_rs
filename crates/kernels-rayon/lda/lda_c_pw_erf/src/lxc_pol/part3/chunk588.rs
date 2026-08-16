//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 588/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk588(t3276: f64, t3213: f64, t3217: f64, t3220: f64, t3224: f64, t3228: f64, t3231: f64, t3253: f64, t3260: f64, t3264: f64, t3269: f64, t3271: f64, t3275: f64, t426: f64) -> (f64, f64) {
    let t3277 = 1.9486833333333333_f64 * t3276;
    let t3278 = -8.81424_f64 * t3213 - 2.93808_f64 * t3217 - 3.0_f64 / 2.0_f64 * t3220 - 6.0_f64 * t426 * t3224 - 2.0_f64 / 3.0_f64 * t3228 + t3231 / 2.0_f64 - t426 * t3253 / 2.0_f64 - 1.46904_f64 * t3260 + 2.20356_f64 * t3264 + t3269 + t3271 - t3275 - t3277;
    (t3277, t3278)
}
