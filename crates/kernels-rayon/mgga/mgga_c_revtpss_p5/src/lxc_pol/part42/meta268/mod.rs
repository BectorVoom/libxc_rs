//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta268 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1015;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1016;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta268(t3943: f64, t794: f64, t1412: f64, t159: f64, t216: f64, t1408: f64, t2482: f64, t596: f64, t3981: f64, t212: f64, t225: f64, t816: f64, t3995: f64, t2681: f64, t820: f64, t1416: f64, t124: f64, t2237: f64, t800: f64, t1376: f64, t123: f64, t125: f64, t2452: f64, t9720: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9744, t9748, t9765, t9766, t9775) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1015(t3943, t794, t1412, t159, t216, t1408, t2482, t596, t3981, t212, t225, t816);
        let (t9776, t9779, t9780, t9784, t9786, t9789) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1016(t3995, t9775, t1408, t2681, t820, t1416, t124, t212, t2237, t800, t1376, t123, t125, t2452, t9720);
    (t9744, t9748, t9765, t9766, t9775, t9776, t9779, t9780, t9784, t9786, t9789)
}
