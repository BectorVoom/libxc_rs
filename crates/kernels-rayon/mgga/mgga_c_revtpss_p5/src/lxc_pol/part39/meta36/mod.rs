//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta36 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk224;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk225;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk226;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk227;
use chunk4::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk228;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta36(t138: f64, t697: f64, t687: f64, t689: f64, t693: f64, t146: f64, t682: f64, t36: f64, t37: f64, t157: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let t698 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk224(t138, t697);
        let (t700, t701) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk225(t687, t689, t693, t698, t146);
        let (t702, t704) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk226(t700, t701, t682);
        let t705 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk227(t36, t37);
        let t706 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk228(t157, t705);
    (t698, t700, t701, t702, t704, t705, t706)
}
