//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta691 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2434;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2435;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta691(t1358: f64, t588: f64, t9647: f64, t4086: f64, t9646: f64, t1399: f64, t22: f64, t555: f64, t1429: f64, t39501: f64, t1419: f64, t5744: f64, t786: f64, t1398: f64, t268: f64, t4101: f64, t543: f64, t793: f64, t544: f64, t9989: f64, t4003: f64, t10013: f64, t2453: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46388, t46389, t46392, t46412, t46456) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2434(t1358, t588, t9647, t4086, t9646, t1399, t22, t555, t1429, t39501, t1419, t5744);
        let (t46457, t46463, t46475, t46478, t46495) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2435(t46456, t786, t1398, t268, t4101, t543, t793, t544, t9989, t4003, t10013, t2453);
    (t46388, t46389, t46392, t46412, t46457, t46463, t46475, t46478, t46495)
}
