//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta262 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1457;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1458;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1459;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta262(t1414: f64, t828: f64, t9628: f64, t221: f64, t3889: f64, t3979: f64, t3978: f64, t1408: f64, t2482: f64, t596: f64, t3981: f64, t3923: f64, t550: f64, t543: f64, t3992: f64, t2661: f64, t212: f64, t225: f64, t816: f64, t3995: f64, t2681: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9757, t9761, t9762, t9765) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1457(t1414, t828, t9628, t221, t3889, t3979, t3978, t1408, t2482, t596);
        let (t9766, t9768, t9769, t9770, t9771, t9775) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1458(t3981, t9765, t3923, t550, t543, t3992, t2661, t212, t225, t596, t816);
        let (t9776, t9779) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1459(t3995, t9775, t1408, t2681, t820);
    (t9757, t9761, t9762, t9765, t9766, t9768, t9769, t9770, t9771, t9775, t9776, t9779)
}
