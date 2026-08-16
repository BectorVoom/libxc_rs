//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1176/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1176(t405: f64, t7853: f64, t103: f64, t13399: f64, t13407: f64, t13565: f64, t14106: f64, t14110: f64, t14152: f64, t14162: f64, t14170: f64, t14181: f64, t14183: f64, t1576: f64, t17177: f64, t17185: f64, t17215: f64, t17217: f64, t17222: f64, t17224: f64, t19314: f64, t19349: f64, t19354: f64, t19381: f64, t2060: f64, t3358: f64, t525: f64, t9552: f64) -> f64 {
    let t21216 = t405 * t7853;
    let t21218 = 0.21595_f64 * t17177 - 0.02666666666666667_f64 * t17185 + 0.08_f64 * t17215 + 0.10666666666666667_f64 * t17217 - 0.02666666666666667_f64 * t17222 + 0.0044444444444444444_f64 * t17224 + 0.09597777777777777_f64 * t13399 + 0.11197407407407407_f64 * t13407 - 0.12_f64 * t13565 * t14106 * t19314 + 0.04_f64 * t13565 * t14152 * t19314 - 0.008888888888888889_f64 * t13565 * t14110 * t19314 + 0.044444444444444446_f64 * t14162 + 0.05925925925925926_f64 * t14170 + 0.044444444444444446_f64 * t14181 - 0.007407407407407408_f64 * t14183 + 0.03732469135802469_f64 * t9552 + 0.017777777777777778_f64 * t2060 * t3358 * t19381 + 0.013333333333333334_f64 * t103 * t525 * t19349 - 0.0022222222222222222_f64 * t103 * t1576 * t19354 - 0.02666666666666667_f64 * t21216;
    t21218
}
