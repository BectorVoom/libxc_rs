//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1253/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1253<F: Float>(t20987: F, t20992: F, t20995: F, t20999: F, t21001: F, t21003: F, t21005: F, t21007: F, t21009: F, t21013: F, t21016: F, t21021: F) -> F {
    let t22040 = -t20987 - t20992 - t20995 - t20999 - t21001 + t21003 - t21005 + t21007 + t21009 + t21013 - t21016 - t21021;
    t22040
}
