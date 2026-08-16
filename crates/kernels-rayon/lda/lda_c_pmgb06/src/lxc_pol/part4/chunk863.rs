//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 863/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk863(t3701: f64, t3707: f64, t3713: f64, t3714: f64, t3719: f64, t3727: f64, t3731: f64, t3736: f64, t3744: f64, t3748: f64, t3762: f64, t3764: f64, t4532: f64, t4534: f64, t4537: f64, t4544: f64) -> f64 {
    let t6087 = 0.0004883052614935079_f64 * t4532 - 16.0_f64 * t4534 - t4537 - 0.5848223622634646_f64 * t3701 - t3707 + t3713 + 1.1696447245269292_f64 * t3714 + t3719 - t3727 + t3731 - t3736 - t3744 - 17.315859105681465_f64 * t3748 - t3762 - t3764 - t4544;
    t6087
}
