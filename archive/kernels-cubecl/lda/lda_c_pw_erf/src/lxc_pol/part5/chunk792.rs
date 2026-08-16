//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 792/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk792<F: Float>(t6941: F, t6943: F, t6948: F, t6950: F, t6952: F, t6956: F, t6960: F, t6962: F, t6967: F, t6972: F, t6976: F, t6978: F, t6983: F, t6985: F, t6987: F, t6990: F) -> F {
    let t7277 = t6941 + t6943 + t6948 - t6950 - t6952 - t6956 + t6960 + t6962 + t6967 - t6972 + t6976 - t6978 - t6983 - t6985 + t6987 + t6990;
    t7277
}
