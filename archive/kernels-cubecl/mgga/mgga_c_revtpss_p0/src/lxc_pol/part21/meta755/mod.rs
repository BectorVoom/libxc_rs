//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta755 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2647;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2648;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2649;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2650;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta755<F: Float>(t1353: F, t1883: F, t46825: F, t9793: F, t13848: F, t9810: F, t9816: F, t9818: F, t1408: F, t241: F, t820: F, t2482: F, t814: F, t9991: F, t13805: F, t13847: F, t13789: F, t13790: F, t13804: F, t13944: F, t3924: F, t3934: F, t3936: F, t4056: F, t46800: F, t46804: F, t46810: F, t47248: F, t48595: F, t48686: F, t48687: F, t48691: F, t48692: F, t48696: F, t543: F, t5671: F, t5673: F, t5674: F, t5675: F, t5704: F, t9628: F, t9840: F, t9984: F, t46917: F, t5706: F, t47201: F, t46478: F, t9898: F, t13783: F, t13926: F, t13975: F, t46812: F, t46817: F, t46820: F, t46824: F, t46828: F, t46831: F, t46833: F, t46837: F, t46840: F, t46846: F, t46853: F, t46859: F, t47249: F, t9956: F, t9995: F, t2661: F, t3992: F, t4057: F, t5608: F, t4004: F, t5651: F, t9934: F, t47198: F, t5665: F, t5629: F, t9779: F, t5661: F, t9909: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t48698, t48700, t48709, t48712, t48731) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2647::<F>(t1353, t1883, t46825, t9793, t13848, t9810, t9816, t9818, t1408, t241, t820, t2482, t814, t9991);
        let t48745 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2648::<F>(t13805, t13847, t13848, t48731, t1353, t13789, t13790, t13804, t13944, t3924, t3934, t3936, t4056, t46800, t46804, t46810, t47248, t48595, t48686, t48687, t48691, t48692, t48696, t48700, t48709, t48712, t543, t5671, t5673, t5674, t5675, t5704, t9628, t9840, t9984);
        let (t48760, t48778) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2649::<F>(t46917, t5706, t241, t47201, t820, t46478, t9898, t13783, t13804, t13926, t13975, t3924, t3934, t3936, t46812, t46817, t46820, t46824, t46828, t46831, t46833, t46837, t46840, t46846, t46853, t46859, t47248, t47249, t5673, t5674, t9956, t9995);
        let (t48786, t48790, t48792, t48794, t48796) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2650::<F>(t2661, t3992, t4057, t5608, t4004, t5651, t9934, t47198, t5665, t5629, t9779, t5661, t9909);
    (t48698, t48745, t48760, t48778, t48786, t48790, t48792, t48794, t48796)
}
