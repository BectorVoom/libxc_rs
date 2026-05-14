//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1326/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1326<F: Float>(t10438: F, t17972: F, t17975: F, t17976: F, t17978: F, t17980: F, t17982: F, t17984: F, t17986: F, t17987: F, t17992: F, t17996: F, t18001: F, t18006: F, t18007: F, t18008: F, t18009: F) -> (F,) {
    let t19283 = -t17972 + t17975 - t17976 + t17978 - t17980 - t17982 + t17984 + t17986 - t17987 + t17992 - t17996 + t18001 + t18006 - t18007 + t18008 + t18009 - t10438;
    (t19283,)
}
