//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta104 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk556;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk557;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta104(t2453: f64, t252: f64, t136: f64, t257: f64, t124: f64, t137: f64, t68: f64, t786: f64, t861: f64, t789: f64, t867: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2454, t2455, t2456, t2457) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk556(t2453, t252, t136, t257, t124, t137, t68);
        let (t2458, t2460, t2461, t2462, t2464, t2465) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk557(t2455, t2457, t2454, t786, t861, t789, t252, t867);
    (t2454, t2455, t2456, t2457, t2458, t2460, t2461, t2462, t2464, t2465)
}
