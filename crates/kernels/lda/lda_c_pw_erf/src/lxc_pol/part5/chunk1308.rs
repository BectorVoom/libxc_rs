//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1308/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1308<F: Float>(t20993: F, t20994: F, t21001: F, t21002: F, t21003: F, t21004: F, t21005: F, t21008: F, t21012: F, t21014: F, t21015: F, t21016: F, t21019: F) -> F {
    let t23210 = -t20993 + t20994 + t21001 - t21002 - t21003 + t21004 - t21005 + t21008 - t21012 - t21014 + t21015 - t21016 - t21019;
    t23210
}
