//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta694 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2440;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2441;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta694(t1401: f64, t46722: f64, t159: f64, t216: f64, t4010: f64, t1386: f64, t2482: f64, t2668: f64, t1376: f64, t40757: f64, t2681: f64, t4000: f64, t820: f64, t10111: f64, t1408: f64, t9720: f64, t1353: f64, t1414: f64, t685: f64, t40735: f64, t535: f64, t235: f64, t5744: f64, t2453: f64, t1389: f64, t268: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46723, t46730, t46740, t46760, t46766) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2440(t1401, t46722, t159, t216, t4010, t1386, t2482, t2668, t1376, t40757, t2681, t4000, t820);
        let (t46784, t46786, t46787, t46800, t46801, t46802, t46808) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2441(t10111, t1408, t9720, t1353, t1414, t685, t40735, t535, t235, t5744, t2453, t1389, t268);
    (t46723, t46730, t46740, t46760, t46766, t46784, t46786, t46787, t46800, t46801, t46802, t46808)
}
