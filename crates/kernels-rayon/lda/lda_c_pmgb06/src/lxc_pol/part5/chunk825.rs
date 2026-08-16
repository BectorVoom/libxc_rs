//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 825/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk825(t3032: f64, t7811: f64, t137: f64, t132: f64, t6837: f64, t6839: f64, t6841: f64, t6844: f64, t6846: f64, t5497: f64, t6852: f64, t3368: f64, t3380: f64, t4909: f64, t6800: f64, t6811: f64, t6819: f64, t6873: f64, t6875: f64, t6877: f64, t7596: f64, t7614: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7812 = t3032 * t7811;
    let t7813 = t137 * t7812;
    let t7815 = t132 * t7813 / 5.0_f64;
    let t7816 = t6837 / 15.0_f64;
    let t7817 = t6839 / 15.0_f64;
    let t7818 = 2.0_f64 / 15.0_f64 * t6841;
    let t7819 = t6844 / 15.0_f64;
    let t7820 = t6846 / 15.0_f64;
    let t7821 = 2.0_f64 / 135.0_f64 * t5497;
    let t7822 = 2.0_f64 / 15.0_f64 * t6852;
    let t7832 = -0.03999074074074074_f64 * t7596 - 0.035991666666666665_f64 * t7614 + 0.023994444444444443_f64 * t6800 - 0.07198333333333333_f64 * t6811 + 0.035991666666666665_f64 * t6819 - 0.02666666666666667_f64 * t6873 + 0.013333333333333334_f64 * t6875 + 0.0044444444444444444_f64 * t6877 - t3368 - t3380 - 0.022222222222222223_f64 * t4909;
    (t7812, t7813, t7815, t7816, t7817, t7818, t7819, t7820, t7821, t7822, t7832)
}
