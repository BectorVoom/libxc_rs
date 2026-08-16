//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1255/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1255(t493: f64, t5463: f64, t6503: f64, t2501: f64, t3213: f64, t2979: f64, t6782: f64, t1444: f64, t6783: f64, t464: f64, t6123: f64, t1386: f64, t439: f64) -> (f64, f64, f64, f64, f64) {
    let t16505 = 4.0_f64 / 9.0_f64 * t493 * t5463 * t6503;
    let t16506 = t3213 * t2501;
    let t16507 = 4.0_f64 / 405.0_f64 * t16506;
    let t16510 = 2.0_f64 / 45.0_f64 * t493 * t2979 * t6782;
    let t16512 = 2.0_f64 / 45.0_f64 * t1444 * t6783;
    let t16513 = t6123 * t464;
    let t16516 = 2.0_f64 / 45.0_f64 * t439 * t16513 * t1386;
    (t16505, t16507, t16510, t16512, t16516)
}
