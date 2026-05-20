//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta443 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1694;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1695;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1696;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1697;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1698;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta443<F: Float>(t2289: F, t2367: F, t10210: F, t625: F, t10214: F, t10255: F, t10207: F, t111: F, t2340: F, t2366: F, t39455: F, t36227: F, t2350: F, t2256: F, t36415: F, t2358: F, t2362: F, t100: F, t101: F, t10217: F, t10227: F, t10229: F, t10232: F, t10233: F, t10236: F, t10237: F, t10241: F, t10246: F, t10250: F, t10344: F, t105: F, t108: F, t2344: F, t2349: F, t2351: F, t2354: F, t2357: F, t656: F, t659: F, t97: F, tau0: F, t10208: F, t10213: F, t10254: F, t2339: F, t46143: F, t46144: F, t46146: F, t655: F, t69: F, t114: F, t10259: F, t10260: F, t10263: F, t10416: F, t10426: F, t118: F, t1310: F, t1312: F, t13207: F, t13216: F, t13435: F, t1453: F, t2322: F, t2331: F, t2371: F, t3813: F, t4254: F, t43735: F, t45923: F, t46125: F, t46126: F, t46129: F, t46137: F, t508: F, t5523: F, t569: F, t651: F, t670: F, t93: F, t3860: F, t4029: F, t3857: F, t4038: F, t9387: F, t2608: F, t3850: F, t512: F, t39454: F, t521: F, t1333: F, t9413: F) -> (F, F, F, F, F, F, F, F) {
        let (t46148, t46150, t46152, t46154, t46157, t46158, t46166, t46173, t46196) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1694::<F>(t2289, t2367, t10210, t625, t10214, t10255, t10207, t111, t2340, t2366, t39455, t36227);
        let t46228 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1695::<F>(t2350, t2256, t36415, t2358, t2362, t100, t101, t10217, t10227, t10229, t10232, t10233, t10236, t10237, t10241, t10246, t10250, t10344, t105, t108, t2344, t2349, t2351, t2354, t2357, t46173, t46196, t656, t659, t97, tau0);
        let t46232 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1696::<F>(t10208, t10213, t10254, t2339, t2340, t2366, t46143, t46144, t46146, t46148, t46150, t46152, t46154, t46157, t46158, t46166, t46228, t655, t69);
        let (t46233, t46250) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1697::<F>(t114, t46232, t10259, t10260, t10263, t10416, t10426, t118, t1310, t1312, t13207, t13216, t13435, t1453, t2322, t2331, t2371, t3813, t4254, t43735, t45923, t46125, t46126, t46129, t46137, t508, t5523, t569, t651, t670, t93);
        let (t46280, t46282, t46287, t46290, t46292, t46297) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1698::<F>(t3860, t4029, t3857, t4038, t9387, t2608, t3850, t512, t39454, t521, t1333, t9413);
    (t46233, t46250, t46280, t46282, t46287, t46290, t46292, t46297)
}
