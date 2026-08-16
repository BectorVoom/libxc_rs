//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta791 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2883;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2884;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta791(t10039: f64, t2439: f64, t2777: f64, t1429: f64, t39501: f64, t4056: f64, t9994: f64, t10014: f64, t10136: f64, t215: f64, t3923: f64, t268: f64, t4101: f64, t543: f64, t10023: f64, t4003: f64, t1419: f64, t5744: f64, t786: f64, t1398: f64, t793: f64, t10073: f64, t10084: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46401, t46412, t46416, t46443, t46445, t46448) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2883(t10039, t2439, t2777, t1429, t39501, t4056, t9994, t10014, t10136, t215, t3923, t268, t4101, t543);
        let (t46452, t46456, t46457, t46463, t46465) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2884(t10023, t268, t4003, t46445, t1419, t5744, t786, t1398, t4101, t543, t793, t10073, t10084);
    (t46401, t46412, t46416, t46443, t46448, t46452, t46456, t46457, t46463, t46465)
}
