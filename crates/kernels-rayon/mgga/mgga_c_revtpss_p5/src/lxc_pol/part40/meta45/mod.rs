//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta45 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk278;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk279;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk280;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk281;
use chunk4::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk282;
use chunk5::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk283;
use chunk6::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk284;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta45(t212: f64, t225: f64, t233: f64, t251: f64, t689: f64, t234: f64, t786: f64, t72: f64, t686: f64, t822: f64, t837: f64, t860: f64, t213: f64, t820: f64, t868: f64, t783: f64, t791: f64, t862: f64, t865: f64, t261: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t869 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk278(t212, t225);
        let (t870, t871, t873, t874) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk279(t233, t251, t869, t689, t234, t786);
        let (t875, t878, t879) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk280(t251, t72, t686, t874, t822);
        let t886 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk281(t837, t879, t234, t860, t213, t820, t873, t878);
        let t887 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk282(t868, t886);
        let t890 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk283(t213, t783, t791, t862, t865, t887);
        let t892 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk284(t261);
    (t869, t870, t871, t873, t874, t875, t878, t879, t886, t887, t890, t892)
}
