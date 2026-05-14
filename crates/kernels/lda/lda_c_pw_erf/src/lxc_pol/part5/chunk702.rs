//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 702/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk702<F: Float>(t525: F, t6988: F, t2478: F, t581: F, t593: F, t1466: F, t1318: F, t6941: F, t6943: F, t6948: F, t6950: F, t6952: F, t6956: F, t6960: F, t6962: F, t6967: F, t6972: F, t6976: F, t6978: F, t6983: F, t6985: F, t6987: F) -> (F, F, F, F, F, F) {
    let t6990 = 8.0 / 45.0 * t6988 * t525;
    let t6991 = t581 * t2478;
    let t6992 = t6991 * t593;
    let t6993 = t1466 * t6992;
    let t6995 = 4.0 / 15.0 * t1318 * t6993;
    let t6996 = t6941 + t6943 + t6948 - t6950 - t6952 - t6956 + t6960 + t6962 + t6967 - t6972 + t6976 - t6978 - t6983 - t6985 + t6987 + t6990 - t6995;
    (t6990, t6991, t6992, t6993, t6995, t6996)
}
