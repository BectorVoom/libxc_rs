//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1287/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1287<F: Float>(t15961: F, t15964: F, t15967: F, t15969: F, t15970: F, t15974: F, t15979: F, t15981: F, t15984: F, t15986: F, t15988: F, t15989: F, t15990: F, t15991: F, t15996: F, t15998: F, t16002: F) -> (F,) {
    let t19118 = -t15961 + t15964 + t15967 - t15969 + t15970 - t15974 - t15979 - t15981 - t15984 - t15986 - t15988 + t15989 - t15990 - t15991 + t15996 + t15998 + t16002;
    (t19118,)
}
