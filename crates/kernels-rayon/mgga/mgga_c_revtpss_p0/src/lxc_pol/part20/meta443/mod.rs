//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta443 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1694;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1695;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1696;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1697;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1698;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta443(t2289: f64, t2367: f64, t10210: f64, t625: f64, t10214: f64, t10255: f64, t10207: f64, t111: f64, t2340: f64, t2366: f64, t39455: f64, t36227: f64, t2350: f64, t2256: f64, t36415: f64, t2358: f64, t2362: f64, t100: f64, t101: f64, t10217: f64, t10227: f64, t10229: f64, t10232: f64, t10233: f64, t10236: f64, t10237: f64, t10241: f64, t10246: f64, t10250: f64, t10344: f64, t105: f64, t108: f64, t2344: f64, t2349: f64, t2351: f64, t2354: f64, t2357: f64, t656: f64, t659: f64, t97: f64, tau0: f64, t10208: f64, t10213: f64, t10254: f64, t2339: f64, t46143: f64, t46144: f64, t46146: f64, t655: f64, t69: f64, t114: f64, t10259: f64, t10260: f64, t10263: f64, t10416: f64, t10426: f64, t118: f64, t1310: f64, t1312: f64, t13207: f64, t13216: f64, t13435: f64, t1453: f64, t2322: f64, t2331: f64, t2371: f64, t3813: f64, t4254: f64, t43735: f64, t45923: f64, t46125: f64, t46126: f64, t46129: f64, t46137: f64, t508: f64, t5523: f64, t569: f64, t651: f64, t670: f64, t93: f64, t3860: f64, t4029: f64, t3857: f64, t4038: f64, t9387: f64, t2608: f64, t3850: f64, t512: f64, t39454: f64, t521: f64, t1333: f64, t9413: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46148, t46150, t46152, t46154, t46157, t46158, t46166, t46173, t46196) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1694(t2289, t2367, t10210, t625, t10214, t10255, t10207, t111, t2340, t2366, t39455, t36227);
        let t46228 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1695(t2350, t2256, t36415, t2358, t2362, t100, t101, t10217, t10227, t10229, t10232, t10233, t10236, t10237, t10241, t10246, t10250, t10344, t105, t108, t2344, t2349, t2351, t2354, t2357, t46173, t46196, t656, t659, t97, tau0);
        let t46232 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1696(t10208, t10213, t10254, t2339, t2340, t2366, t46143, t46144, t46146, t46148, t46150, t46152, t46154, t46157, t46158, t46166, t46228, t655, t69);
        let (t46233, t46250) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1697(t114, t46232, t10259, t10260, t10263, t10416, t10426, t118, t1310, t1312, t13207, t13216, t13435, t1453, t2322, t2331, t2371, t3813, t4254, t43735, t45923, t46125, t46126, t46129, t46137, t508, t5523, t569, t651, t670, t93);
        let (t46280, t46282, t46287, t46290, t46292, t46297) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1698(t3860, t4029, t3857, t4038, t9387, t2608, t3850, t512, t39454, t521, t1333, t9413);
    (t46233, t46250, t46280, t46282, t46287, t46290, t46292, t46297)
}
