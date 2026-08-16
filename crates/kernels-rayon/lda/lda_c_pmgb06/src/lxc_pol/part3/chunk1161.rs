//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1161/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1161(t1380: f64, t1586: f64, t1831: f64, t1981: f64, t1912: f64, t3198: f64, t1444: f64, t4728: f64, t4732: f64, t4602: f64, t5442: f64, t1911: f64, t493: f64, t9925: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13861 = 2.0_f64 / 15.0_f64 * t1981 * t1380 * t1831 * t1586;
    let t13863 = t3198 * t1912 / 15.0_f64;
    let t13865 = 2.0_f64 / 15.0_f64 * t1444 * t4728;
    let t13867 = t1444 * t4732 / 15.0_f64;
    let t13869 = 4.0_f64 / 15.0_f64 * t4602 * t5442;
    let t13872 = t493 * t9925 * t1911 / 15.0_f64;
    (t13861, t13863, t13865, t13867, t13869, t13872)
}
