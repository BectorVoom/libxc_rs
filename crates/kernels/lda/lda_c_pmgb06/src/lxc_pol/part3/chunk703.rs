//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 703/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk703<F: Float>(t3765: F, t283: F, t3701: F, t3707: F, t3713: F, t3714: F, t3719: F, t3721: F, t3727: F, t3731: F, t3736: F, t3744: F, t3746: F, t3748: F, t3762: F, t3764: F, t4515: F) -> F {
    let t4544 = F::new(4.0) * t3765;
    let t4547 = -F::new(1.1696447245269292) * t3701 - t3707 + t3713 + F::new(2.3392894490538585) * t3714 + t3719 - F::new(0.00018311447306006544) * t3721 - t3727 + t3731 - t3736 - t3744 - F::new(0.5848223622634646) * t3746 - F::new(34.63171821136293) * t3748 - t3762 - t3764 + t4544 + F::new(0.0197516734986138) * t4515 * t283;
    t4547
}
