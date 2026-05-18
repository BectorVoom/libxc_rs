//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 861/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk861<F: Float>(t3317: F, t3319: F, t3335: F, t3342: F, t3692: F, t3695: F, t3696: F, t3697: F, t3943: F, t3944: F, t3945: F, t7851: F, t7855: F) -> F {
    let t8918 = F::new(12.0) * t7851 + F::new(12.0) * t7855 - F::new(0.821419393556371) * t3335 - F::new(0.5476129290375806) * t3342 + t3943 + t3944 - t3945 + t3692 + t3695 + t3696 - t3697 + F::new(0.821419393556371) * t3317 + F::new(0.821419393556371) * t3319;
    t8918
}
