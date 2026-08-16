//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta39 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk238;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk239;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk240;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk241;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk242;
use chunk5::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk243;
use chunk6::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk244;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta39(t45: f64, t606: f64, t766: f64, t81: f64, zeta_threshold: f64, t57: f64, t212: f64, t251: f64, t225: f64, t257: f64, t689: f64, t211: f64, t209: f64, t252: f64, t72: f64, t686: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t769, t770) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk238(t45, t606, t766, t81, zeta_threshold);
        let t775 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk239(t57, t606, t770, t769, zeta_threshold);
        let t779 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk240(t212, t251);
        let t780 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk241(t225, t257);
        let (t781, t783, t784, t785) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk242(t779, t780, t689, t211);
        let t786 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk243(t209, t785);
        let (t787, t788, t789) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk244(t252, t786, t257, t72, t686);
    (t770, t775, t779, t780, t781, t783, t784, t785, t786, t787, t788, t789)
}
