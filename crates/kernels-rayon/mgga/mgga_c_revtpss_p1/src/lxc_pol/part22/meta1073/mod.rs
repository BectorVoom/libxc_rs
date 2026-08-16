//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1073 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3848;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3849;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3850;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3851;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1073(t21969: f64, t221: f64, t3978: f64, t3979: f64, t4010: f64, t6816: f64, t1353: f64, t13767: f64, t2661: f64, t22027: f64, t9775: f64, t22252: f64, t3992: f64, t543: f64, t550: f64, t22263: f64, t1412: f64, t6861: f64, t3938: f64, t5608: f64, t5659: f64, t1399: f64, t22025: f64, t13902: f64, t13907: f64, t1392: f64, t1394: f64, t22229: f64, t22237: f64, t22240: f64, t22245: f64, t22246: f64, t22249: f64, t3829: f64, t3889: f64, t4045: f64, t539: f64, t5644: f64, t5650: f64, t5652: f64, t6837: f64, t6840: f64, t73: f64, t73578: f64, t73991: f64, t39419: f64, t39422: f64, t46289: f64, t46297: f64, t46963: f64, t73314: f64, t73315: f64, t73316: f64, t73317: f64, t73322: f64, t73327: f64, t73328: f64, t73330: f64, t73332: f64, t73333: f64, t73334: f64, t73338: f64, t39483: f64, t39520: f64, t39528: f64, t39531: f64, t46970: f64, t73339: f64, t73342: f64, t73350: f64, t73353: f64, t73354: f64, t73355: f64, t73356: f64, t73357: f64, t73358: f64, t73361: f64, t73364: f64, t73365: f64, t73366: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t74010, t74012, t74015, t74017, t74022) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3848(t21969, t221, t3978, t3979, t4010, t6816, t1353, t13767, t2661, t22027, t9775, t22252, t3992, t543, t550);
        let (t74024, t74026, t74029, t74033, t74037, t74077) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3849(t22263, t9775, t1412, t6861, t2661, t3938, t3992, t5608, t5659, t1399, t22025, t1353, t13902, t13907, t1392, t1394, t21969, t22229, t22237, t22240, t22245, t22246, t22249, t3829, t3889, t4045, t539, t5644, t5650, t5652, t6837, t6840, t73, t73578, t73991, t74012);
        let t74099 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3850(t39419, t39422, t46289, t46297, t46963, t73314, t73315, t73316, t73317, t73322, t73327, t73328, t73330, t73332, t73333, t73334, t73338);
        let t74100 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3851(t39483, t39520, t39528, t39531, t46970, t73339, t73342, t73350, t73353, t73354, t73355, t73356, t73357, t73358, t73361, t73364, t73365, t73366);
    (t74010, t74015, t74017, t74022, t74024, t74026, t74029, t74033, t74037, t74077, t74099, t74100)
}
