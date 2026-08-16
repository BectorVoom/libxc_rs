//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta112 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk576;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk577;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk578;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta112(t2777: f64, t870: f64, t2439: f64, t123: f64, t212: f64, t676: f64, t225: f64, t822: f64, t251: f64, t836: f64, t231: f64, t233: f64, t860: f64, t869: f64, t689: f64, t136: f64, t2457: f64, t2710: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2778, t2780, t2782) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk576(t2777, t870, t2439, t123, t212, t676);
        let t2783 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk577(t225, t822);
        let (t2786, t2787, t2789, t2790, t2791, t2793, t2796) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk578(t251, t836, t231, t2783, t2782, t233, t860, t869, t689, t136, t2457, t2710);
    (t2778, t2780, t2782, t2783, t2786, t2787, t2789, t2790, t2791, t2793, t2796)
}
