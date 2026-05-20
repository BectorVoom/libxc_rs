//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta39 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk288;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk289;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk290;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk291;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk292;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk293;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta39<F: Float>(t779: F, t780: F, t689: F, t211: F, t209: F, t252: F, t257: F, t72: F, t686: F, t579: F, t65: F, t64: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t781, t783, t784, t785) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk288::<F>(t779, t780, t689, t211);
        let t786 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk289::<F>(t209, t785);
        let (t787, t788) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk290::<F>(t252, t786, t257, t72);
        let t789 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk291::<F>(t686, t788);
        let (t791, t793) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk292::<F>(t787, t789, t579, t65);
        let t794 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk293::<F>(t64, t793);
    (t781, t783, t784, t785, t786, t787, t788, t789, t791, t793, t794)
}
