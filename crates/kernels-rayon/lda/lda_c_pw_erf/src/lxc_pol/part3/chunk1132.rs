//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1132/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1132(t13252: f64, t3863: f64, t4769: f64, t571: f64, t4872: f64, t4819: f64, t9278: f64, t1472: f64, t5307: f64, t1308: f64, t1333: f64, t2065: f64, t951: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13253 = 16.0_f64 / 45.0_f64 * t13252;
    let t13255 = t571 * t3863 * t4769;
    let t13256 = 16.0_f64 / 45.0_f64 * t13255;
    let t13258 = t571 * t3863 * t4872;
    let t13259 = 8.0_f64 / 45.0_f64 * t13258;
    let t13261 = t571 * t9278 * t4819;
    let t13262 = 8.0_f64 / 27.0_f64 * t13261;
    let t13264 = 8.0_f64 / 15.0_f64 * t1472 * t5307;
    let t13269 = 8.0_f64 / 15.0_f64 * t571 * t1308 * t2065 * t1333 * t951;
    (t13253, t13256, t13259, t13262, t13264, t13269)
}
