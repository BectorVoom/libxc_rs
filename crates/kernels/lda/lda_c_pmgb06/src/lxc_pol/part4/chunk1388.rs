//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1388/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1388<F: Float>(t15798: F, t15799: F, t15801: F, t15802: F, t15803: F, t15804: F, t15805: F, t15808: F, t15810: F, t15815: F, t15817: F, t15819: F, t15821: F, t15824: F, t15828: F) -> F {
    let t18182 = t15798 + t15799 + t15801 + t15802 - t15803 - t15804 + t15805 - t15808 + t15810 - t15815 - t15817 - t15819 - t15821 - t15824 - t15828;
    t18182
}
