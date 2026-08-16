//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta24 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk155;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk156;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk157;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk158;
use chunk4::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk159;
use chunk5::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk160;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta24(t221: f64, t462: f64, t65: f64, t225: f64, t460: f64, t355: f64, t424: f64, t452: f64, t454: f64, sigma2: f64, t51: f64, t52: f64, rho1: f64, t414: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t464, t467, t471) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk155(t221, t462, t65, t225, t460, t355, t424, t452, t454);
        let (t472, t473) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk156(t471);
        let t474 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk157(sigma2);
        let (t475, t476, t479) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk158(t473, t474, t51, t52, rho1);
        let t480 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk159(t475, t479);
        let (t481, t482) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk160(t467, t480, t414);
    (t464, t467, t471, t472, t473, t474, t475, t476, t479, t480, t481, t482)
}
