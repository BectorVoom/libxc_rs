//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 869/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk869<F: Float>(t675: F, t682: F, t696: F, t8719: F, t3711: F, t971: F, t1066: F, t1101: F, t3934: F, t643: F, t1026: F, t1035: F, t1041: F) -> (F, F, F, F, F) {
    let t8723 = F::new(0.5848223622634646) * t696 * t675 * t8719 * t682;
    let t8724 = t971 * t3711;
    let t8727 = F::new(120.0) * t1101 * t1066;
    let t8729 = t643 * t3934;
    let t8733 = F::new(36.0) * t1041 * t1026 * t1035;
    (t8723, t8724, t8727, t8729, t8733)
}
