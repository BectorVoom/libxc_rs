//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta23 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk149;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk150;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk151;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk152;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta23(t406: f64, t409: f64, t412: f64, t416: f64, t439: f64, t300: f64, t424: f64, t426: f64, t435: f64, t344: f64, t56: f64, t404: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t444, t447, t448) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk149(t406, t409, t412, t416);
        let (t452, t454, t456, t458, t459) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk150(t439, t448, t300, t424, t426, t435, t406);
        let t460 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk151(t456, t459);
        let (t461, t462) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk152(t344, t56, t404);
    (t444, t447, t448, t452, t454, t456, t458, t459, t460, t461, t462)
}
