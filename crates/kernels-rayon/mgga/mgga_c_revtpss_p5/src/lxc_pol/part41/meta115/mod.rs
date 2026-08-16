//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta115 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk586;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk587;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta115(t910: f64, t914: f64, t287: f64, t913: f64, t275: f64, t273: f64, t276: f64, t2846: f64, t240: f64, t68: f64, t281: f64, t283: f64, t698: f64, t931: f64, t1014: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2869, t2873, t2874, t2880, t2884, t2892, t2897, t2902, t2904) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk586(t910, t914, t287, t913, t275, t273, t276, t2846, t240, t68, t281, t283);
        let (t2905, t2906, t2908) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk587(t2904, t698, t931, t1014, t240);
    (t2869, t2873, t2874, t2880, t2884, t2892, t2897, t2902, t2904, t2905, t2906, t2908)
}
