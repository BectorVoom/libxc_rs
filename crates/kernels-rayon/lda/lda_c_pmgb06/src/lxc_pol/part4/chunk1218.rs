//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1218/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1218(t16051: f64, t1447: f64, t6403: f64, t6504: f64, t5499: f64, t6407: f64, t1444: f64, t6399: f64, t2979: f64, t493: f64, t6398: f64, t1380: f64, t1586: f64, t2545: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16052 = 8.0_f64 / 135.0_f64 * t16051;
    let t16053 = t1447 * t6403;
    let t16054 = 8.0_f64 / 45.0_f64 * t16053;
    let t16055 = t1447 * t6504;
    let t16056 = 8.0_f64 / 27.0_f64 * t16055;
    let t16057 = t5499 * t6407;
    let t16058 = 8.0_f64 / 27.0_f64 * t16057;
    let t16060 = 4.0_f64 / 45.0_f64 * t1444 * t6399;
    let t16063 = 4.0_f64 / 45.0_f64 * t493 * t2979 * t6398;
    let t16067 = 2.0_f64 / 45.0_f64 * t493 * t1380 * t2545 * t1586;
    (t16052, t16054, t16056, t16058, t16060, t16063, t16067)
}
