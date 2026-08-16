//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta447 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1706;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1707;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1708;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1709;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta447(t1398: f64, t4056: f64, t543: f64, t1437: f64, t2482: f64, t686: f64, t72: f64, t10014: f64, t10136: f64, t215: f64, t3923: f64, t268: f64, t4101: f64, t10023: f64, t4003: f64, t10119: f64, t1419: f64, t5744: f64, t786: f64, t10026: f64, t793: f64, t10073: f64, t10084: f64, t10059: f64, t10130: f64, t3924: f64, t4057: f64, t5745: f64, t5755: f64, t820: f64, t9840: f64, t555: f64, t9898: f64, t14192: f64, t2782: f64, t9994: f64, t544: f64, t9989: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46432, t46433, t46435, t46443, t46445, t46448) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1706(t1398, t4056, t543, t1437, t2482, t686, t72, t10014, t10136, t215, t3923, t268, t4101);
        let (t46452, t46454, t46458, t46463) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1707(t10023, t268, t4003, t46445, t10014, t10119, t1419, t5744, t786, t10026, t1398, t4101, t543, t793);
        let t46467 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1708(t10073, t10084, t10059, t10130, t3924, t4057, t46435, t46443, t46448, t46452, t46454, t46458, t46463, t5745, t5755, t820, t9840);
        let (t46469, t46472, t46475, t46476, t46477, t46479, t46483, t46490) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1709(t555, t9898, t14192, t2782, t9994, t544, t9989, t3923, t4003, t215, t268, t4056, t4101, t543);
    (t46432, t46433, t46467, t46469, t46472, t46475, t46476, t46477, t46479, t46483, t46490)
}
