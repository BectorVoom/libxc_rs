//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1582/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1582<F: Float>(t1357: F, t22975: F, t689: F, t5599: F, t6896: F, t6919: F, t5741: F, t74892: F, t22315: F, t48084: F, t22858: F, t47372: F, t686: F, t72: F) -> (F, F, F, F, F, F) {
    let t86314 = t689 * t1357 * t22975;
    let t86317 = t689 * t5599 * t6896;
    let t86346 = t689 * t5599 * t6919;
    let t86350 = t74892 * t5741;
    let t86354 = t48084 * t22315;
    let t86358 = t47372 * t22858 * t72 * t686;
    (t86314, t86317, t86346, t86350, t86354, t86358)
}
