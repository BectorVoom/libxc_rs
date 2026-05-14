//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1017/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1017<F: Float>(t6070: F, t638: F, t6068: F, t643: F, t11145: F, t11147: F, t11149: F, t11155: F, t11157: F, t11160: F, t11162: F, t8814: F, t8822: F, t8824: F, t8826: F, t8830: F, t8834: F) -> (F,) {
    let t15024 = t638 * t6070;
    let t15026 = t638 * t6068;
    let t15028 = t643 * t6068;
    let t15030 = t643 * t6070;
    let t15038 = -0.0003662289461201309 * t11145 - 103.89515463408878 * t11147 - 64.0 * t11149 + 8.0 * t15024 + 8.0 * t15026 - 8.0 * t15028 - 8.0 * t15030 + t8814 + t8822 - 0.5848223622634646 * t8824 - 17.315859105681465 * t8826 + t8830 + t8834 + 0.0009766105229870158 * t11155 - 0.0011393789434848518 * t11157 + 4.678578898107717 * t11160 + 207.79030926817757 * t11162;
    (t15038,)
}
