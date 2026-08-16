//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta292 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1709;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1710;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1711;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta292(t3981: f64, t9765: f64, t3923: f64, t550: f64, t543: f64, t3992: f64, t2661: f64, t212: f64, t225: f64, t596: f64, t816: f64, t3995: f64, t1408: f64, t2681: f64, t820: f64, t1416: f64, t124: f64, t2237: f64, t800: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9766, t9768, t9769, t9770, t9771, t9775) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1709(t3981, t9765, t3923, t550, t543, t3992, t2661, t212, t225, t596, t816);
        let (t9776, t9779) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1710(t3995, t9775, t1408, t2681, t820);
        let (t9780, t9784) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1711(t1416, t9779, t124, t212, t2237, t800);
    (t9766, t9768, t9769, t9770, t9771, t9775, t9776, t9779, t9780, t9784)
}
