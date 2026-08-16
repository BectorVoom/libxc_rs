//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1292/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1292<F: Float>(t16952: F, t16954: F, t16956: F, t16959: F, t16961: F, t16963: F, t16965: F, t16967: F, t16969: F, t16971: F, t16972: F, t16974: F, t16975: F, t16976: F) -> F {
    let t16977 = -t16952 - t16954 - t16956 - t16959 - t16961 + t16963 + t16965 + t16967 + t16969 + t16971 + t16972 + t16974 - t16975 - t16976;
    t16977
}
