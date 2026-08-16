//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1168/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1168(t1894: f64, t3213: f64, t1423: f64, t5365: f64, t13921: f64, t13923: f64, t13926: f64, t13929: f64, t13932: f64, t13936: f64, t13938: f64, t13941: f64, t13943: f64, t13947: f64) -> (f64, f64, f64) {
    let t13948 = t3213 * t1894;
    let t13949 = 2.0_f64 / 135.0_f64 * t13948;
    let t13950 = t1423 * t5365;
    let t13951 = 4.0_f64 / 45.0_f64 * t13950;
    let t13952 = t13921 - t13923 - t13926 - t13929 + t13932 - t13936 + t13938 + t13941 + t13943 - t13947 + t13949 + t13951;
    (t13949, t13951, t13952)
}
