//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta753 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2635;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2636;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2637;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta753<F: Float>(t13774: F, t2661: F, t5675: F, t9934: F, t1868: F, t4056: F, t1882: F, t2682: F, t4000: F, t5677: F, t820: F, t13985: F, t46740: F, t13783: F, t13789: F, t1388: F, t1390: F, t13944: F, t1410: F, t36776: F, t3889: F, t3934: F, t3938: F, t4012: F, t46645: F, t46649: F, t46652: F, t48143: F, t48438: F, t48445: F, t48449: F, t48453: F, t48458: F, t5591: F, t5671: F, t828: F, t9628: F, t9955: F, t9956: F, t1872: F, t3924: F, t9816: F, t9818: F, t13848: F, t47274: F, t13878: F, t9765: F, t13869: F, t3989: F, t5608: F, t9840: F) -> (F, F, F, F, F, F, F, F) {
        let (t48462, t48466, t48475, t48487, t48488) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2635::<F>(t13774, t2661, t5675, t9934, t1868, t4056, t1882, t2682, t4000, t5677, t820, t13985, t46740);
        let t48490 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2636::<F>(t48488, t13783, t13789, t1388, t1390, t13944, t1410, t1868, t36776, t3889, t3934, t3938, t4012, t46645, t46649, t46652, t48143, t48438, t48445, t48449, t48453, t48458, t48462, t48466, t48475, t48487, t5591, t5671, t5675, t828, t9628, t9955, t9956);
        let (t48494, t48498, t48509, t48510, t48514) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2637::<F>(t1872, t3924, t9816, t9818, t13848, t47274, t9956, t13878, t9765, t13869, t3989, t2661, t5608, t9840, t9934);
    (t48466, t48475, t48490, t48494, t48498, t48509, t48510, t48514)
}
