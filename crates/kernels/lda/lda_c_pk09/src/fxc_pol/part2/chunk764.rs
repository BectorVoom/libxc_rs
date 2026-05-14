//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 764/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk764<F: Float>(t3317: F, t3319: F, t3335: F, t3342: F, t3692: F, t3695: F, t3696: F, t3697: F, t3943: F, t3944: F, t3945: F, t7851: F, t7855: F, t7888: F, t3323: F, t3326: F, t3706: F, t3713: F, t3715: F, t3961: F, t3962: F, t3963: F, t7870: F, t7875: F, t7879: F, t7884: F) -> (F, F) {
    let t8918 = 12.0 * t7851 + 12.0 * t7855 - 0.821419393556371 * t3335 - 0.5476129290375806 * t3342 + t3943 + t3944 - t3945 + t3692 + t3695 + t3696 - t3697 + 0.821419393556371 * t3317 + 0.821419393556371 * t3319;
    let t8926 = 24.0 * t7888;
    let t8927 = 0.5476129290375806 * t3323 + 0.5476129290375806 * t3326 + t3706 + 24.0 * t7870 - 24.0 * t7875 + 24.0 * t7879 - 24.0 * t7884 + t8926 + t3961 + t3962 - t3963 + t3713 + t3715;
    (t8918, t8927)
}
