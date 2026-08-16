//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1111/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1111(t13204: f64, t2007: f64, t3220: f64, t1962: f64, t3254: f64, t439: f64, t835: f64, t9271: f64, t1977: f64, t3226: f64, t1447: f64, t4605: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13205 = 2.0_f64 / 45.0_f64 * t13204;
    let t13206 = t3220 * t2007;
    let t13207 = 4.0_f64 / 45.0_f64 * t13206;
    let t13210 = t439 * t1962 * t3254 / 45.0_f64;
    let t13211 = t9271 * t835;
    let t13212 = 2.0_f64 / 45.0_f64 * t13211;
    let t13213 = t3226 * t1977;
    let t13214 = 4.0_f64 / 45.0_f64 * t13213;
    let t13215 = t1447 * t4605;
    (t13205, t13207, t13210, t13212, t13214, t13215)
}
