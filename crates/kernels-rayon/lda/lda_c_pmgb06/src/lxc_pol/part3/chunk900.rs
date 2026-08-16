//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 900/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk900(t3408: f64, t405: f64, t3405: f64, t134: f64, t147: f64, t1531: f64, t1626: f64, t955: f64, t2060: f64, t474: f64, t1147: f64, t135: f64, t146: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9685 = t405 * t3408;
    let t9687 = t405 * t3405;
    let t9693 = t147 / t134 / t1531;
    let t9700 = t955 * t1626;
    let t9702 = t2060 * t474;
    let t9712 = t1147 * t147;
    let t9715 = 0.10864197530864197_f64 * t146 * t9712 * t135;
    (t9685, t9687, t9693, t9700, t9702, t9712, t9715)
}
