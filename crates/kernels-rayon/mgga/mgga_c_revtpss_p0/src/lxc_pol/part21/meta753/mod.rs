//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta753 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2635;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2636;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2637;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta753(t13774: f64, t2661: f64, t5675: f64, t9934: f64, t1868: f64, t4056: f64, t1882: f64, t2682: f64, t4000: f64, t5677: f64, t820: f64, t13985: f64, t46740: f64, t13783: f64, t13789: f64, t1388: f64, t1390: f64, t13944: f64, t1410: f64, t36776: f64, t3889: f64, t3934: f64, t3938: f64, t4012: f64, t46645: f64, t46649: f64, t46652: f64, t48143: f64, t48438: f64, t48445: f64, t48449: f64, t48453: f64, t48458: f64, t5591: f64, t5671: f64, t828: f64, t9628: f64, t9955: f64, t9956: f64, t1872: f64, t3924: f64, t9816: f64, t9818: f64, t13848: f64, t47274: f64, t13878: f64, t9765: f64, t13869: f64, t3989: f64, t5608: f64, t9840: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48462, t48466, t48475, t48487, t48488) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2635(t13774, t2661, t5675, t9934, t1868, t4056, t1882, t2682, t4000, t5677, t820, t13985, t46740);
        let t48490 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2636(t48488, t13783, t13789, t1388, t1390, t13944, t1410, t1868, t36776, t3889, t3934, t3938, t4012, t46645, t46649, t46652, t48143, t48438, t48445, t48449, t48453, t48458, t48462, t48466, t48475, t48487, t5591, t5671, t5675, t828, t9628, t9955, t9956);
        let (t48494, t48498, t48509, t48510, t48514) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2637(t1872, t3924, t9816, t9818, t13848, t47274, t9956, t13878, t9765, t13869, t3989, t2661, t5608, t9840, t9934);
    (t48466, t48475, t48490, t48494, t48498, t48509, t48510, t48514)
}
