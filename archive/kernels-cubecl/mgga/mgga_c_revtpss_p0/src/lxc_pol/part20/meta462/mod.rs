//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta462 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1756;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1757;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1758;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1759;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta462<F: Float>(t47187: F, t543: F, t820: F, t843: F, t9991: F, t9997: F, t1386: F, t2237: F, t2482: F, t4021: F, t235: F, t46475: F, t239: F, t1353: F, t1388: F, t1390: F, t3934: F, t3936: F, t3944: F, t46479: F, t46483: F, t46682: F, t46918: F, t46922: F, t46924: F, t46931: F, t46934: F, t46941: F, t46944: F, t46947: F, t46949: F, t46951: F, t5671: F, t5673: F, t5675: F, t800: F, t828: F, t9699: F, t9805: F, t9810: F, t9826: F, t9955: F, t9993: F, t4000: F, t596: F, t10003: F, t1412: F, t3923: F, t2661: F, t9835: F, t9934: F, t9914: F, t9918: F, t221: F, t4018: F, t4019: F, t9899: F, t4059: F, t9909: F, t9812: F, t9962: F, t13845: F, t46751: F, t9818: F, t13847: F, t9819: F, t9840: F, t9958: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t47188, t47195, t47199, t47201) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1756::<F>(t47187, t543, t820, t843, t9991, t9997, t1386, t2237, t2482, t4021, t235, t46475);
        let t47212 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1757::<F>(t239, t47201, t820, t1353, t1388, t1390, t3934, t3936, t3944, t46479, t46483, t46682, t46918, t46922, t46924, t46931, t46934, t46941, t46944, t46947, t46949, t46951, t47188, t47195, t47199, t5671, t5673, t5675, t800, t828, t9699, t9805, t9810, t9826, t9955, t9993);
        let (t47216, t47218, t47221, t47223, t47227) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1758::<F>(t2482, t4000, t596, t10003, t1412, t3923, t2661, t9835, t9934, t9914, t9918, t221, t4018, t4019, t9899);
        let (t47229, t47231, t47235, t47239, t47245) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1759::<F>(t4059, t9909, t9812, t9962, t13845, t46751, t9818, t9835, t13847, t9819, t9840, t9958);
    (t47188, t47212, t47216, t47218, t47221, t47223, t47227, t47229, t47231, t47235, t47239, t47245)
}
