//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1073 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3848;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3849;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3850;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3851;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1073<F: Float>(t21969: F, t221: F, t3978: F, t3979: F, t4010: F, t6816: F, t1353: F, t13767: F, t2661: F, t22027: F, t9775: F, t22252: F, t3992: F, t543: F, t550: F, t22263: F, t1412: F, t6861: F, t3938: F, t5608: F, t5659: F, t1399: F, t22025: F, t13902: F, t13907: F, t1392: F, t1394: F, t22229: F, t22237: F, t22240: F, t22245: F, t22246: F, t22249: F, t3829: F, t3889: F, t4045: F, t539: F, t5644: F, t5650: F, t5652: F, t6837: F, t6840: F, t73: F, t73578: F, t73991: F, t39419: F, t39422: F, t46289: F, t46297: F, t46963: F, t73314: F, t73315: F, t73316: F, t73317: F, t73322: F, t73327: F, t73328: F, t73330: F, t73332: F, t73333: F, t73334: F, t73338: F, t39483: F, t39520: F, t39528: F, t39531: F, t46970: F, t73339: F, t73342: F, t73350: F, t73353: F, t73354: F, t73355: F, t73356: F, t73357: F, t73358: F, t73361: F, t73364: F, t73365: F, t73366: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t74010, t74012, t74015, t74017, t74022) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3848::<F>(t21969, t221, t3978, t3979, t4010, t6816, t1353, t13767, t2661, t22027, t9775, t22252, t3992, t543, t550);
        let (t74024, t74026, t74029, t74033, t74037, t74077) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3849::<F>(t22263, t9775, t1412, t6861, t2661, t3938, t3992, t5608, t5659, t1399, t22025, t1353, t13902, t13907, t1392, t1394, t21969, t22229, t22237, t22240, t22245, t22246, t22249, t3829, t3889, t4045, t539, t5644, t5650, t5652, t6837, t6840, t73, t73578, t73991, t74012);
        let t74099 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3850::<F>(t39419, t39422, t46289, t46297, t46963, t73314, t73315, t73316, t73317, t73322, t73327, t73328, t73330, t73332, t73333, t73334, t73338);
        let t74100 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3851::<F>(t39483, t39520, t39528, t39531, t46970, t73339, t73342, t73350, t73353, t73354, t73355, t73356, t73357, t73358, t73361, t73364, t73365, t73366);
    (t74010, t74015, t74017, t74022, t74024, t74026, t74029, t74033, t74037, t74077, t74099, t74100)
}
