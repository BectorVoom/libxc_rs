//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1199/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1199(t2002: f64, t5268: f64, t5273: f64, t1444: f64, t6770: f64, t10293: f64, t493: f64, t6769: f64, t1586: f64, t2541: f64, t2991: f64, t5499: f64, t6536: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15817 = 2.0_f64 / 45.0_f64 * t2002 * t5268;
    let t15819 = 2.0_f64 / 27.0_f64 * t2002 * t5273;
    let t15821 = 2.0_f64 / 27.0_f64 * t1444 * t6770;
    let t15824 = 2.0_f64 / 27.0_f64 * t493 * t10293 * t6769;
    let t15828 = t493 * t2991 * t2541 * t1586 / 27.0_f64;
    let t15829 = t5499 * t6536;
    (t15817, t15819, t15821, t15824, t15828, t15829)
}
