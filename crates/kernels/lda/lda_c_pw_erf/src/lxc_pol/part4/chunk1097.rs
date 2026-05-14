//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1097/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1097<F: Float>(t4804: F, t6981: F, t11991: F, t12015: F, t12017: F, t15956: F, t15958: F, t15961: F, t15964: F, t15967: F, t15969: F, t15970: F, t15971: F, t15974: F, t15979: F, t15981: F, t15984: F, t15986: F) -> (F, F, F, F, F) {
    let t15988 = 8.0 / 15.0 * t4804 * t6981;
    let t15989 = 64.0 / 45.0 * t11991;
    let t15990 = 32.0 / 135.0 * t12015;
    let t15991 = 64.0 / 135.0 * t12017;
    let t15992 = t15956 + t15958 - t15961 + t15964 + t15967 - t15969 + t15970 + 4.0 / 3.0 * t15971 - t15974 - t15979 - t15981 - t15984 - t15986 - t15988 + t15989 - t15990 - t15991;
    (t15988, t15989, t15990, t15991, t15992)
}
