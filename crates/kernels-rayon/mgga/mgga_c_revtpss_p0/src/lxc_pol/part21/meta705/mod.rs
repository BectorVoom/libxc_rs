//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta705 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2530;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2531;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta705(t1419: f64, t4056: f64, t1429: f64, t39501: f64, t1398: f64, t9840: f64, t2482: f64, t4114: f64, t686: f64, t72: f64, t543: f64, t1437: f64, t10014: f64, t10136: f64, t215: f64, t3923: f64, t268: f64, t4101: f64, t10023: f64, t4003: f64, t10119: f64, t5744: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46407, t46412, t46422, t46424, t46432, t46433, t46435) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2530(t1419, t4056, t1429, t39501, t1398, t9840, t2482, t4114, t686, t72, t543, t1437);
        let (t46443, t46448, t46452, t46454, t46456) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2531(t10014, t10136, t215, t3923, t268, t4101, t543, t10023, t4003, t10119, t1419, t5744);
    (t46407, t46412, t46422, t46424, t46432, t46433, t46435, t46443, t46448, t46452, t46454, t46456)
}
