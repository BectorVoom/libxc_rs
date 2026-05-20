//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta39 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk238;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk239;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk240;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk241;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk242;
use chunk5::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk243;
use chunk6::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk244;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta39<F: Float>(t45: F, t606: F, t766: F, t81: F, zeta_threshold: F, t57: F, t212: F, t251: F, t225: F, t257: F, t689: F, t211: F, t209: F, t252: F, t72: F, t686: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t769, t770) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk238::<F>(t45, t606, t766, t81, zeta_threshold);
        let t775 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk239::<F>(t57, t606, t770, t769, zeta_threshold);
        let t779 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk240::<F>(t212, t251);
        let t780 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk241::<F>(t225, t257);
        let (t781, t783, t784, t785) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk242::<F>(t779, t780, t689, t211);
        let t786 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk243::<F>(t209, t785);
        let (t787, t788, t789) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk244::<F>(t252, t786, t257, t72, t686);
    (t770, t775, t779, t780, t781, t783, t784, t785, t786, t787, t788, t789)
}
