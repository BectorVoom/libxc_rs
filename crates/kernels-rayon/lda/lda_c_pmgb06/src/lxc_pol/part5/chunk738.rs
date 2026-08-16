//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 738/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk738(t6868: f64, t6903: f64, t518: f64, t166: f64, t161: f64, t1925: f64, t1972: f64, t2555: f64, t3451: f64, t439: f64, t486: f64, t493: f64, t5497: f64, t5500: f64, t6783: f64, t6788: f64, t6791: f64, t6833: f64, t6837: f64, t6839: f64, t6841: f64, t6844: f64, t6846: f64, t6852: f64) -> (f64, f64, f64, f64) {
    let t6904 = t6868 + t6903;
    let t6905 = t518 * t6904;
    let t6906 = t166 * t6905;
    let t6909 = -t493 * t6783 / 45.0_f64 - 2.0_f64 / 45.0_f64 * t1972 * t1925 - 2.0_f64 / 45.0_f64 * t439 * t6788 - 2.0_f64 / 45.0_f64 * t493 * t6791 + t486 * t2555 / 30.0_f64 + t161 * t6833 / 30.0_f64 + t6837 / 45.0_f64 + t6839 / 45.0_f64 + 2.0_f64 / 45.0_f64 * t6841 + t6844 / 45.0_f64 + t6846 / 45.0_f64 - 4.0_f64 / 405.0_f64 * t5497 - 8.0_f64 / 135.0_f64 * t5500 + t3451 / 135.0_f64 + 2.0_f64 / 45.0_f64 * t6852 - t161 * t6906 / 30.0_f64;
    (t6904, t6905, t6906, t6909)
}
