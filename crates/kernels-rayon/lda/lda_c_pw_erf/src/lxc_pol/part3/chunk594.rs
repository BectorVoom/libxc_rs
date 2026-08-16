//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 594/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk594(t133: f64, t3227: f64, t3230: f64, t1568: f64, t411: f64, t3219: f64, t1870: f64, t1871: f64, t3224: f64, t3253: f64, t3269: f64, t3271: f64, t3275: f64, t3277: f64, t3280: f64, t3284: f64, t3302: f64, t3305: f64, t3322: f64, t3325: f64, t3348: f64) -> (f64, f64, f64, f64, f64) {
    let t3349 = t133 * t3227;
    let t3351 = t133 * t3230;
    let t3357 = t411 * t1568;
    let t3361 = t133 * t3219;
    let t3363 = -t3348 - 2.2990066666666666_f64 * t3349 + 1.724255_f64 * t3351 - 1.724255_f64 * t133 * t3253 - 20.69106_f64 * t133 * t3224 + 15.518295_f64 * t1870 * t1871 * t3357 - 5.172765_f64 * t3361 - t3284 + t3269 + t3280 + t3271 - t3275 - t3277 - t3322 - t3302 + t3325 - t3305;
    (t3349, t3351, t3357, t3361, t3363)
}
