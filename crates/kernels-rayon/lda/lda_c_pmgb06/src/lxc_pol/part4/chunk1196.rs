//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1196/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1196(t1420: f64, t6379: f64, t439: f64, t5225: f64, t6185: f64, t15395: f64, t1897: f64, t436: f64, t6705: f64, t1517: f64, t2592: f64, t11875: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15786 = 4.0_f64 / 15.0_f64 * t1420 * t6379;
    let t15789 = 4.0_f64 / 15.0_f64 * t439 * t5225 * t6185;
    let t15792 = 2.0_f64 / 15.0_f64 * t439 * t1897 * t15395;
    let t15793 = t6705 * t436;
    let t15794 = 2.0_f64 / 45.0_f64 * t15793;
    let t15795 = t2592 * t1517;
    let t15796 = 2.0_f64 / 45.0_f64 * t15795;
    let t15797 = 8.0_f64 / 135.0_f64 * t11875;
    (t15786, t15789, t15792, t15794, t15796, t15797)
}
