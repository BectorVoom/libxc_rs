//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1157/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1157(t13548: f64, t4729: f64, t511: f64, t10409: f64, t13524: f64, t13528: f64, t13531: f64, t13534: f64, t13537: f64, t13539: f64, t13541: f64, t13543: f64, t13545: f64, t13547: f64) -> (f64, f64, f64) {
    let t13549 = 8.0_f64 / 15.0_f64 * t13548;
    let t13550 = t511 * t4729;
    let t13551 = 4.0_f64 / 45.0_f64 * t13550;
    let t13553 = t13524 - t13528 - t13531 - t13534 - t13537 - t13539 + t13541 + t13543 + t13545 - t13547 + t13549 + t13551 + 12.0_f64 * t10409;
    (t13549, t13551, t13553)
}
