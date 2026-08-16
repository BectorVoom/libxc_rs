//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1236/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1236(t132: f64, t435: f64, t6599: f64, t432: f64, t6613: f64, t486: f64, t6906: f64, t16255: f64, t16259: f64, t16262: f64, t16263: f64, t16264: f64, t16266: f64, t16271: f64, t16275: f64, t16276: f64, t16277: f64, t16278: f64, t16282: f64) -> (f64, f64, f64, f64) {
    let t16284 = t132 * t435 * t6599;
    let t16285 = 4.0_f64 / 45.0_f64 * t16284;
    let t16286 = t432 * t6613;
    let t16287 = 4.0_f64 / 45.0_f64 * t16286;
    let t16289 = t486 * t6906 / 15.0_f64;
    let t16290 = t16255 + t16259 + t16262 + t16263 + t16264 - t16266 - t16271 - t16275 + t16276 - t16277 - t16278 - t16282 - t16285 - t16287 - t16289;
    (t16285, t16287, t16289, t16290)
}
