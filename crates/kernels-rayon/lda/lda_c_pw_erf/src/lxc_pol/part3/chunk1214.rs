//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1214/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1214(t4804: f64, t4959: f64, t3794: f64, t1278: f64, t1325: f64, t4956: f64, t4957: f64, t1341: f64, t5327: f64, t2171: f64, t3724: f64, t4953: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14321 = 8.0_f64 / 5.0_f64 * t4804 * t4959;
    let t14323 = 8.0_f64 / 5.0_f64 * t3794 * t4959;
    let t14327 = 4.0_f64 / 5.0_f64 * t1325 * t4956 * t4957 * t1278;
    let t14329 = 8.0_f64 / 15.0_f64 * t5327 * t1341;
    let t14331 = 8.0_f64 / 9.0_f64 * t2171 * t3724;
    let t14333 = 8.0_f64 / 5.0_f64 * t4804 * t4953;
    (t14321, t14323, t14327, t14329, t14331, t14333)
}
