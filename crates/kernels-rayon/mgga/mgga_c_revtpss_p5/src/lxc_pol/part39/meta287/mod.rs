//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta287 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1033;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1034;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta287(t20: f64, t596: f64, t12: f64, t583: f64, t27: f64, t2231: f64, t2237: f64, t592: f64, t2236: f64, t3: f64, t25: f64, t2240: f64, t602: f64, t2246: f64, t599: f64, t88: f64, t89: f64, t90: f64, t29: f64, t46: f64, t47: f64, t58: f64, t59: f64, t10199: f64, t2851: f64, t78: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10284, t10287, t10288, t10290, t10295, t10298) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1033(t20, t596, t12, t583, t27, t2231, t2237, t592, t2236, t3, t25, t2240, t602);
        let (t10301, t10309, t10355, t10368, t10379, t10389) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1034(t2246, t599, t88, t89, t90, t29, t46, t47, t58, t59, t10199, t2851, t78);
    (t10284, t10287, t10288, t10290, t10295, t10298, t10301, t10309, t10355, t10368, t10379, t10389)
}
