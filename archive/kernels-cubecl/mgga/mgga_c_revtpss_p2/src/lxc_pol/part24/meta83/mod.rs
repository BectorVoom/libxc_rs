//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta83 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk491;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk492;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk493;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk494;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta83<F: Float>(t2552: F, t164: F, t172: F, t2538: F, t123: F, t147: F, t2434: F, t143: F, t680: F, t130: F, t700: F, t701: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2553, t2554, t2555, t2556) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk491::<F>(t2552, t164, t172);
        let (t2557, t2562) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk492::<F>(t2538, t2556, t123, t147, t2434);
        let (t2563, t2564, t2565, t2566) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk493::<F>(t143, t680, t130, t700);
        let (t2567, t2569) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk494::<F>(t2566, t701, t2565);
    (t2553, t2554, t2555, t2556, t2557, t2562, t2563, t2564, t2565, t2566, t2567, t2569)
}
