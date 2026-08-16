//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1253/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1253(t12092: f64, t153: f64, t1859: f64, t439: f64, t4659: f64, t6498: f64, t13715: f64, t4645: f64, t2010: f64, t4655: f64, t1444: f64, t6504: f64) -> (f64, f64, f64, f64, f64) {
    let t16487 = 4.0_f64 / 27.0_f64 * t439 * t12092 * t153 * t1859;
    let t16490 = 2.0_f64 / 27.0_f64 * t439 * t6498 * t4659;
    let t16491 = t13715 * t153;
    let t16494 = 16.0_f64 / 81.0_f64 * t439 * t16491 * t4645;
    let t16497 = 8.0_f64 / 27.0_f64 * t2010 * t6498 * t4655;
    let t16499 = 4.0_f64 / 9.0_f64 * t1444 * t6504;
    (t16487, t16490, t16494, t16497, t16499)
}
