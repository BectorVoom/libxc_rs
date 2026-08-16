//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta789 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2879;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2880;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta789(t36227: f64, t36415: f64, t3860: f64, t4029: f64, t3857: f64, t4038: f64, t9387: f64, t2608: f64, t3850: f64, t512: f64, t1333: f64, t9413: f64, t3853: f64, t30: f64, t513: f64, t9603: f64, t33: f64, t516: f64, t9615: f64, t10153: f64, t2435: f64, t2439: f64, t3895: f64, t4078: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46196, t46212, t46279, t46281, t46286, t46289, t46297) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2879(t36227, t36415, t3860, t4029, t3857, t4038, t9387, t2608, t3850, t512, t1333, t9413);
        let (t46302, t46310, t46328, t46353, t46356) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2880(t3853, t3860, t30, t513, t9603, t33, t516, t9615, t10153, t2435, t2439, t3895, t4078);
    (t46196, t46212, t46279, t46281, t46286, t46289, t46297, t46302, t46310, t46328, t46353, t46356)
}
