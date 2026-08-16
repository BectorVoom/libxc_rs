//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta541 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1589;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1590;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1591;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1592;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta541(t1450: f64, t23059: f64, t22809: f64, t566: f64, t4147: f64, t23087: f64, t9593: f64, t6836: f64, t1921: f64, t6936: f64, t1913: f64, t6951: f64, t25072: f64, t571: f64, t5891: f64, t5915: f64, t5911: f64, t5895: f64, t5823: f64, t5907: f64, t22: f64, t39454: f64, t100: f64, t105: f64, t108: f64, t109: f64, t1507: f64, t1510: f64, t21835: f64, t21860: f64, t22604: f64, t22608: f64, t22618: f64, t22621: f64, t22624: f64, t22625: f64, t22699: f64, t2349: f64, t2357: f64, t4269: f64, t4279: f64, t46196: f64, t46212: f64, t5902: f64, t5908: f64, t5912: f64, t97: f64, tau1: f64, t21820: f64, t22628: f64, t2339: f64, t4263: f64, t46143: f64, t46157: f64, t49698: f64, t655: f64, t69: f64, t75540: f64, t75639: f64, t75822: f64, t75831: f64, t75843: f64, t114: f64, t5876: f64, t5883: f64, t1519: f64, t18245: f64, t1843: f64, t22578: f64, t22633: f64, t22634: f64, t22639: f64, t30138: f64, t4248: f64, t508: f64, t5884: f64, t5887: f64, t5920: f64, t5921: f64, t651: f64, t6765: f64, t75941: f64, t7732: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t86731, t86819, t86825, t86828, t86839, t86897, t86903) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1589(t1450, t23059, t22809, t566, t4147, t23087, t9593, t6836, t1921, t6936, t1913, t6951);
        let (t86909, t86981, t86988, t87028, t87046) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1590(t25072, t571, t5891, t5915, t5911, t5895, t5823, t5907, t22, t39454, t100, t105, t108, t109, t1507, t1510, t21835, t21860, t22604, t22608, t22618, t22621, t22624, t22625, t22699, t2349, t2357, t4269, t4279, t46196, t46212, t5902, t5908, t5912, t97, tau1);
        let t87050 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1591(t21820, t22628, t2339, t4263, t46143, t46157, t49698, t5915, t655, t69, t75540, t75639, t75822, t75831, t75843, t86981, t86988, t87046);
        let (t87051, t87064, t87071) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1592(t114, t87050, t5876, t5883, t1519, t18245, t1843, t22578, t22633, t22634, t22639, t30138, t4248, t508, t5884, t5887, t5920, t5921, t651, t6765, t75941, t7732);
    (t86731, t86819, t86825, t86828, t86839, t86897, t86903, t86909, t87028, t87051, t87064, t87071)
}
