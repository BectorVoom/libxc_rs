//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1144/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1144(t6070: f64, t638: f64, t6068: f64, t643: f64, t11145: f64, t11147: f64, t11149: f64, t11155: f64, t11157: f64, t11160: f64, t11162: f64, t8814: f64, t8822: f64, t8824: f64, t8826: f64, t8830: f64, t8834: f64) -> f64 {
    let t15024 = t638 * t6070;
    let t15026 = t638 * t6068;
    let t15028 = t643 * t6068;
    let t15030 = t643 * t6070;
    let t15038 = -0.0003662289461201309_f64 * t11145 - 103.89515463408878_f64 * t11147 - 64.0_f64 * t11149 + 8.0_f64 * t15024 + 8.0_f64 * t15026 - 8.0_f64 * t15028 - 8.0_f64 * t15030 + t8814 + t8822 - 0.5848223622634646_f64 * t8824 - 17.315859105681465_f64 * t8826 + t8830 + t8834 + 0.0009766105229870158_f64 * t11155 - 0.0011393789434848518_f64 * t11157 + 4.678578898107717_f64 * t11160 + 207.79030926817757_f64 * t11162;
    t15038
}
