//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3288/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3288<F: Float>(t5599: F, t689: F, t6919: F, t5741: F, t74892: F, t22315: F, t48084: F, t22858: F, t47372: F, t686: F, t72: F, t1432: F, t22964: F) -> (F, F, F, F, F) {
    let t86346 = t689 * t5599 * t6919;
    let t86350 = t74892 * t5741;
    let t86354 = t48084 * t22315;
    let t86358 = t47372 * t22858 * t72 * t686;
    let t86374 = t1432 * t22964 * t72 * t686;
    (t86346, t86350, t86354, t86358, t86374)
}
