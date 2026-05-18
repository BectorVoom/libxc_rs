//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 670/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk670<F: Float>(t3701: F, t3707: F, t3713: F, t3714: F, t3719: F, t3727: F, t3731: F, t3736: F, t3744: F, t3748: F, t3762: F, t3764: F, t4532: F, t4534: F, t4537: F, t4544: F) -> F {
    let t6087 = F::new(0.0004883052614935079) * t4532 - F::new(16.0) * t4534 - t4537 - F::new(0.5848223622634646) * t3701 - t3707 + t3713 + F::new(1.1696447245269292) * t3714 + t3719 - t3727 + t3731 - t3736 - t3744 - F::new(17.315859105681465) * t3748 - t3762 - t3764 - t4544;
    t6087
}
