//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta117 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk589;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk590;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta117(t240: f64, t2681: f64, t243: f64, t247: f64, t237: f64, t124: f64, t212: f64, t596: f64, t800: f64, t810: f64, t775: f64, t854: f64, t236: f64, t807: f64, t21: f64, t65: f64, t64: f64, t159: f64, t222: f64, t794: f64, t798: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2682, t2686, t2689) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk589(t240, t2681, t243, t247, t237, t124, t212, t596, t800);
        let (t2691, t2693, t2694, t2695, t2699, t2700, t2702, t2703) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk590(t2689, t810, t775, t854, t236, t807, t21, t65, t64, t159, t222, t794, t798);
    (t2682, t2686, t2689, t2691, t2693, t2694, t2695, t2699, t2700, t2702, t2703)
}
