//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta462 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1756;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1757;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1758;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1759;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta462(t47187: f64, t543: f64, t820: f64, t843: f64, t9991: f64, t9997: f64, t1386: f64, t2237: f64, t2482: f64, t4021: f64, t235: f64, t46475: f64, t239: f64, t1353: f64, t1388: f64, t1390: f64, t3934: f64, t3936: f64, t3944: f64, t46479: f64, t46483: f64, t46682: f64, t46918: f64, t46922: f64, t46924: f64, t46931: f64, t46934: f64, t46941: f64, t46944: f64, t46947: f64, t46949: f64, t46951: f64, t5671: f64, t5673: f64, t5675: f64, t800: f64, t828: f64, t9699: f64, t9805: f64, t9810: f64, t9826: f64, t9955: f64, t9993: f64, t4000: f64, t596: f64, t10003: f64, t1412: f64, t3923: f64, t2661: f64, t9835: f64, t9934: f64, t9914: f64, t9918: f64, t221: f64, t4018: f64, t4019: f64, t9899: f64, t4059: f64, t9909: f64, t9812: f64, t9962: f64, t13845: f64, t46751: f64, t9818: f64, t13847: f64, t9819: f64, t9840: f64, t9958: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47188, t47195, t47199, t47201) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1756(t47187, t543, t820, t843, t9991, t9997, t1386, t2237, t2482, t4021, t235, t46475);
        let t47212 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1757(t239, t47201, t820, t1353, t1388, t1390, t3934, t3936, t3944, t46479, t46483, t46682, t46918, t46922, t46924, t46931, t46934, t46941, t46944, t46947, t46949, t46951, t47188, t47195, t47199, t5671, t5673, t5675, t800, t828, t9699, t9805, t9810, t9826, t9955, t9993);
        let (t47216, t47218, t47221, t47223, t47227) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1758(t2482, t4000, t596, t10003, t1412, t3923, t2661, t9835, t9934, t9914, t9918, t221, t4018, t4019, t9899);
        let (t47229, t47231, t47235, t47239, t47245) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1759(t4059, t9909, t9812, t9962, t13845, t46751, t9818, t9835, t13847, t9819, t9840, t9958);
    (t47188, t47212, t47216, t47218, t47221, t47223, t47227, t47229, t47231, t47235, t47239, t47245)
}
