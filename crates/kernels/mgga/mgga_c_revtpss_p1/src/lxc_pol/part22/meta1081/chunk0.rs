//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3894/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3894<F: Float>(t1357: F, t22387: F, t689: F, t3899: F, t6896: F, t1444: F, t2782: F, t4075: F, t556: F, t6918: F, t22453: F, t47530: F) -> (F, F, F, F) {
    let t74810 = t689 * t1357 * t22387;
    let t74813 = t689 * t3899 * t6896;
    let t74824 = t2782 * t556 * t4075 * t6918 * t1444;
    let t74826 = t47530 * t22453;
    (t74810, t74813, t74824, t74826)
}
