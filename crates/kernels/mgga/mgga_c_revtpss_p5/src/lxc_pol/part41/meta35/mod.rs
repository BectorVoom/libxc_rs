//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta35 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk218;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk219;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk220;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk221;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk222;
use chunk5::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk223;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta35<F: Float>(t684: F, t686: F, t123: F, t676: F, t128: F, t72: F, t3: F, t66: F, t124: F, t138: F, t146: F, t682: F, t36: F, t37: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t687, t689) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk218::<F>(t684, t686, t123, t676);
        let (t692, t693, t696, t697) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk219::<F>(t128, t72, t686, t3, t66, t124);
        let t698 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk220::<F>(t138, t697);
        let (t700, t701) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk221::<F>(t687, t689, t693, t698, t146);
        let (t702, t704) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk222::<F>(t700, t701, t682);
        let t705 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk223::<F>(t36, t37);
    (t687, t689, t692, t693, t696, t697, t698, t700, t701, t702, t704, t705)
}
