//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1212/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1212(t10288: f64, t439: f64, t6523: f64, t1444: f64, t6518: f64, t1382: f64, t6134: f64, t11914: f64, t11917: f64, t2948: f64, t6364: f64, t2010: f64, t6371: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15978 = 4.0_f64 / 45.0_f64 * t439 * t10288 * t6523;
    let t15980 = 4.0_f64 / 45.0_f64 * t1444 * t6518;
    let t15982 = 2.0_f64 / 45.0_f64 * t6134 * t1382;
    let t15983 = 4.0_f64 / 135.0_f64 * t11914;
    let t15984 = 4.0_f64 / 135.0_f64 * t11917;
    let t15987 = 4.0_f64 / 45.0_f64 * t439 * t2948 * t6364;
    let t15990 = 8.0_f64 / 45.0_f64 * t2010 * t2948 * t6371;
    (t15978, t15980, t15982, t15983, t15984, t15987, t15990)
}
