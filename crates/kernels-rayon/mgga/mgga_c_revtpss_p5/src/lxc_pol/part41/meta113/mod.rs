//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta113 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk579;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk580;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk581;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk582;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta113(t251: f64, t2783: f64, t786: f64, t231: f64, t268: f64, t675: f64, t836: f64, t72: f64, t860: f64, t686: f64, t874: f64, t2470: f64, t875: f64, t2718: f64, t822: f64, t1941: f64, t271: f64, t689: f64, t907: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2797, t2798) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk579(t251, t2783, t786);
        let (t2801, t2802, t2804, t2806, t2810, t2811) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk580(t231, t268, t675, t836, t2798, t72, t860, t686, t874, t2470, t875, t251, t2718);
        let (t2815, t2846) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk581(t822, t860, t1941, t268, t271);
        let (t2847, t2848) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk582(t2846, t689, t907);
    (t2797, t2798, t2801, t2802, t2804, t2806, t2810, t2811, t2815, t2846, t2847, t2848)
}
