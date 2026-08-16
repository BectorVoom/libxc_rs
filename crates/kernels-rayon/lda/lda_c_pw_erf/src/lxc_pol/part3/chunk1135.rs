//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1135/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1135(t13290: f64, t2017: f64, t571: f64, t4675: f64, t951: f64, t4758: f64, t3416: f64, t5282: f64, t1318: f64, t3854: f64, t4780: f64, t3794: f64, t5310: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13293 = 16.0_f64 / 3.0_f64 * t571 * t2017 * t13290;
    let t13294 = t4675 * t951;
    let t13297 = 16.0_f64 / 5.0_f64 * t571 * t4758 * t13294;
    let t13298 = t3416 * t5282;
    let t13299 = 32.0_f64 / 45.0_f64 * t13298;
    let t13301 = t1318 * t3854 * t4780;
    let t13302 = 16.0_f64 / 45.0_f64 * t13301;
    let t13303 = t3794 * t5310;
    (t13293, t13294, t13297, t13299, t13302, t13303)
}
