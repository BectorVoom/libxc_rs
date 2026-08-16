//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta541 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1589;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1590;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1591;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1592;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta541<F: Float>(t1450: F, t23059: F, t22809: F, t566: F, t4147: F, t23087: F, t9593: F, t6836: F, t1921: F, t6936: F, t1913: F, t6951: F, t25072: F, t571: F, t5891: F, t5915: F, t5911: F, t5895: F, t5823: F, t5907: F, t22: F, t39454: F, t100: F, t105: F, t108: F, t109: F, t1507: F, t1510: F, t21835: F, t21860: F, t22604: F, t22608: F, t22618: F, t22621: F, t22624: F, t22625: F, t22699: F, t2349: F, t2357: F, t4269: F, t4279: F, t46196: F, t46212: F, t5902: F, t5908: F, t5912: F, t97: F, tau1: F, t21820: F, t22628: F, t2339: F, t4263: F, t46143: F, t46157: F, t49698: F, t655: F, t69: F, t75540: F, t75639: F, t75822: F, t75831: F, t75843: F, t114: F, t5876: F, t5883: F, t1519: F, t18245: F, t1843: F, t22578: F, t22633: F, t22634: F, t22639: F, t30138: F, t4248: F, t508: F, t5884: F, t5887: F, t5920: F, t5921: F, t651: F, t6765: F, t75941: F, t7732: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t86731, t86819, t86825, t86828, t86839, t86897, t86903) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1589::<F>(t1450, t23059, t22809, t566, t4147, t23087, t9593, t6836, t1921, t6936, t1913, t6951);
        let (t86909, t86981, t86988, t87028, t87046) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1590::<F>(t25072, t571, t5891, t5915, t5911, t5895, t5823, t5907, t22, t39454, t100, t105, t108, t109, t1507, t1510, t21835, t21860, t22604, t22608, t22618, t22621, t22624, t22625, t22699, t2349, t2357, t4269, t4279, t46196, t46212, t5902, t5908, t5912, t97, tau1);
        let t87050 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1591::<F>(t21820, t22628, t2339, t4263, t46143, t46157, t49698, t5915, t655, t69, t75540, t75639, t75822, t75831, t75843, t86981, t86988, t87046);
        let (t87051, t87064, t87071) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1592::<F>(t114, t87050, t5876, t5883, t1519, t18245, t1843, t22578, t22633, t22634, t22639, t30138, t4248, t508, t5884, t5887, t5920, t5921, t651, t6765, t75941, t7732);
    (t86731, t86819, t86825, t86828, t86839, t86897, t86903, t86909, t87028, t87051, t87064, t87071)
}
