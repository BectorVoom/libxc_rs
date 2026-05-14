//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 764/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk764<F: Float>(t5566: F, t5595: F, t5898: F, t5913: F, t23: F, t342: F, t24: F, t5582: F, t4042: F, t73: F, t1233: F, t165: F, t842: F, t153: F, t1962: F, t4619: F) -> (F, F, F, F, F, F, F, F) {
    let t5915 = t5566 + t5595 + t5898 + t5913;
    let t5939 = t342 * t23;
    let t6006 = t24 * t5582;
    let t6007 = t4042 * t73;
    let t6018 = t1233 * t5582;
    let t6119 = t165 * t842;
    let t6494 = t1962 * t153;
    let t6498 = t4619 * t153;
    (t5915, t5939, t6006, t6007, t6018, t6119, t6494, t6498)
}
