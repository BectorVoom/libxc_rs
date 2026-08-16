//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 992/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk992(t432: f64, t4966: f64, t132: f64, t435: f64, t4816: f64, t486: f64, t4941: f64, t1730: f64, t2025: f64, t2021: f64, t3209: f64, t439: f64, t6550: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11790 = t432 * t4966 / 10.0_f64;
    let t11792 = t132 * t435 * t4816;
    let t11793 = 2.0_f64 / 15.0_f64 * t11792;
    let t11795 = t486 * t4941 / 5.0_f64;
    let t11796 = t2025 * t1730;
    let t11798 = t2021 * t1730;
    let t11799 = 0.09973633333333333_f64 * t11798;
    let t11802 = t439 * t6550 * t3209 / 5.0_f64;
    (t11790, t11793, t11795, t11796, t11799, t11802)
}
