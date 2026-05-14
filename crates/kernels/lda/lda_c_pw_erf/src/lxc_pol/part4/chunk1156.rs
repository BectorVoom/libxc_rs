//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1156/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1156<F: Float>(t16999: F, t17000: F, t17001: F, t17002: F, t17003: F, t17004: F, t17005: F, t17006: F, t17007: F, t17008: F, t17009: F, t17010: F, t17011: F, t17012: F, t17013: F, t17014: F) -> (F,) {
    let t17015 = t16999 - t17000 + t17001 - t17002 + t17003 - t17004 - t17005 + t17006 + t17007 - t17008 - t17009 + t17010 - t17011 + t17012 + t17013 + t17014;
    (t17015,)
}
