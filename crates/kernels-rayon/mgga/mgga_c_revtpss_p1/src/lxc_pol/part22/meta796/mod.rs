//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta796 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2893;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta796(t46740: f64, t9821: f64, t9769: f64, t9793: f64, t9794: f64, t1376: f64, t40757: f64, t2681: f64, t4000: f64, t820: f64, t4006: f64, t10111: f64, t1408: f64, t9720: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t46741, t46757, t46760, t46766, t46767, t46784) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2893(t46740, t9821, t9769, t9793, t9794, t1376, t40757, t2681, t4000, t820, t4006, t10111, t1408, t9720);
    (t46741, t46757, t46760, t46766, t46767, t46784)
}
