//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 703/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk703(t3765: f64, t283: f64, t3701: f64, t3707: f64, t3713: f64, t3714: f64, t3719: f64, t3721: f64, t3727: f64, t3731: f64, t3736: f64, t3744: f64, t3746: f64, t3748: f64, t3762: f64, t3764: f64, t4515: f64) -> f64 {
    let t4544 = 4.0_f64 * t3765;
    let t4547 = -1.1696447245269292_f64 * t3701 - t3707 + t3713 + 2.3392894490538585_f64 * t3714 + t3719 - 0.00018311447306006544_f64 * t3721 - t3727 + t3731 - t3736 - t3744 - 0.5848223622634646_f64 * t3746 - 34.63171821136293_f64 * t3748 - t3762 - t3764 + t4544 + 0.0197516734986138_f64 * t4515 * t283;
    t4547
}
