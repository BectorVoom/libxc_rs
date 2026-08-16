//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta111 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk575;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk576;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk577;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk578;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta111(t2741: f64, t839: f64, t241: f64, t820: f64, t823: f64, t72: f64, t853: f64, t245: f64, t231: f64, t775: f64, t213: f64, t860: f64, t256: f64, t866: f64, t225: f64, t2435: f64, t871: f64, t785: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2742, t2745) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk575(t2741, t839, t241, t820, t823);
        let (t2746, t2747) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk576(t72, t853, t245);
        let t2749 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk577(t231, t775);
        let (t2765, t2769, t2770, t2776, t2777) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk578(t213, t860, t256, t866, t225, t2435, t871, t785);
    (t2742, t2745, t2746, t2747, t2749, t2765, t2769, t2770, t2776, t2777)
}
