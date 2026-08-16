//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1362/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1362(t1594: f64, t2578: f64, t2864: f64, t439: f64, t1420: f64, t6788: f64, t6775: f64, t2002: f64, t5233: f64, t2497: f64, t3223: f64, t1380: f64, t1831: f64, t1981: f64, t2088: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17902 = 2.0_f64 / 45.0_f64 * t439 * t2864 * t2578 * t1594;
    let t17904 = 4.0_f64 / 45.0_f64 * t1420 * t6788;
    let t17906 = 2.0_f64 / 45.0_f64 * t1420 * t6775;
    let t17908 = 4.0_f64 / 45.0_f64 * t2002 * t5233;
    let t17909 = t3223 * t2497;
    let t17910 = 4.0_f64 / 405.0_f64 * t17909;
    let t17914 = 8.0_f64 / 45.0_f64 * t1981 * t1380 * t1831 * t2088;
    (t17902, t17904, t17906, t17908, t17910, t17914)
}
