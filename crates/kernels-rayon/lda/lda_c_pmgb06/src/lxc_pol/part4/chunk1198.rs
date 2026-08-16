//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1198/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1198(t9350: f64, t9352: f64, t9379: f64, t9381: f64, t11897: f64, t161: f64, t489: f64, t6231: f64, t5110: f64, t831: f64, t1069: f64, t1438: f64, t2648: f64, t2960: f64, t439: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15801 = 8.0_f64 / 405.0_f64 * t9350;
    let t15802 = 8.0_f64 / 405.0_f64 * t9352;
    let t15803 = 2.0_f64 / 405.0_f64 * t9379;
    let t15804 = 2.0_f64 / 243.0_f64 * t9381;
    let t15805 = 4.0_f64 / 135.0_f64 * t11897;
    let t15807 = t161 * t489 * t6231;
    let t15808 = 2.0_f64 / 45.0_f64 * t15807;
    let t15810 = 2.0_f64 / 15.0_f64 * t831 * t5110;
    let t15815 = t439 * t2960 * t2648 * t1438 * t1069 / 27.0_f64;
    (t15801, t15802, t15803, t15804, t15805, t15808, t15810, t15815)
}
