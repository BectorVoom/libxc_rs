//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta274 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1010;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1011;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta274(t3923: f64, t550: f64, t543: f64, t3992: f64, t2661: f64, t212: f64, t225: f64, t596: f64, t816: f64, t3995: f64, t1408: f64, t2681: f64, t820: f64, t1416: f64, t124: f64, t2237: f64, t800: f64, t1376: f64, t123: f64, t125: f64, t2452: f64, t9720: f64, t235: f64, t4086: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9768, t9771, t9775, t9776, t9779) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1010(t3923, t550, t543, t3992, t2661, t212, t225, t596, t816, t3995, t1408, t2681, t820);
        let (t9780, t9784, t9786, t9789, t9791, t9792) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1011(t1416, t9779, t124, t212, t2237, t800, t1376, t123, t125, t2452, t9720, t235, t4086);
    (t9768, t9771, t9775, t9776, t9779, t9780, t9784, t9786, t9789, t9791, t9792)
}
