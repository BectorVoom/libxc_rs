//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta444 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1699;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1700;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1701;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta444(t30: f64, t3889: f64, t3853: f64, t3860: f64, t10179: f64, t4147: f64, t513: f64, t9603: f64, t3834: f64, t2257: f64, t1344: f64, t3874: f64, t39456: f64, t9344: f64, t9605: f64, t9608: f64, zeta_threshold: f64, t33: f64, t516: f64, t9615: f64, t3842: f64, t3351: f64, t1348: f64, t3881: f64, t43744: f64, t9357: f64, t9617: f64, t9620: f64, t1343: f64, t13656: f64, t1448: f64, t198: f64, t3828: f64, t3829: f64, t39419: f64, t39422: f64, t46280: f64, t46282: f64, t46287: f64, t46290: f64, t46292: f64, t46297: f64, t5536: f64, t5541: f64, t9547: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46298, t46303, t46304, t46311, t46317, t46325) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1699(t30, t3889, t3853, t3860, t10179, t4147, t513, t9603, t3834, t2257, t1344, t3874, t39456, t9344, t9605, t9608, zeta_threshold);
        let (t46329, t46335, t46345) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1700(t33, t516, t9615, t3842, t3351, t1348, t3881, t43744, t9357, t9617, t9620, t46325, zeta_threshold);
        let t46349 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1701(t1343, t13656, t1448, t198, t3828, t3829, t3889, t39419, t39422, t46280, t46282, t46287, t46290, t46292, t46297, t46298, t46303, t46304, t46345, t5536, t5541, t9547);
    (t46298, t46303, t46311, t46317, t46329, t46335, t46345, t46349)
}
