//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta121 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk605;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk606;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk607;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta121(t225: f64, t2760: f64, t213: f64, t860: f64, t256: f64, t866: f64, t886: f64, t2435: f64, t871: f64, t785: f64, t870: f64, t2439: f64, t123: f64, t212: f64, t676: f64, t822: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2761, t2765, t2769, t2770, t2771, t2772, t2776, t2777) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk605(t225, t2760, t213, t860, t256, t866, t886, t2435, t871, t785);
        let (t2778, t2780, t2782) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk606(t2777, t870, t2439, t123, t212, t676);
        let t2783 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk607(t225, t822);
    (t2761, t2765, t2769, t2770, t2771, t2772, t2776, t2777, t2778, t2780, t2782, t2783)
}
