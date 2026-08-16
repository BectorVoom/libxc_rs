//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta104 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk716;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk717;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk718;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk719;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta104<F: Float>(t2548: F, t730: F, t722: F, t164: F, t172: F, t2538: F, t123: F, t147: F, t2434: F, t143: F, t680: F, t130: F, t700: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2549, t2552) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk716::<F>(t2548, t730, t722);
        let (t2553, t2554, t2555, t2556) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk717::<F>(t2552, t164, t172);
        let (t2557, t2562) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk718::<F>(t2538, t2556, t123, t147, t2434);
        let (t2563, t2564, t2565, t2566) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk719::<F>(t143, t680, t130, t700);
    (t2549, t2552, t2553, t2554, t2555, t2556, t2557, t2562, t2563, t2564, t2565, t2566)
}
