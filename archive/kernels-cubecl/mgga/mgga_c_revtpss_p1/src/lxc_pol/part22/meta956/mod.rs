//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta956 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3200;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3201;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3202;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3203;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3204;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3205;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3206;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta956<F: Float>(t1464: F, t5789: F, t18177: F, t575: F, t1921: F, t1913: F, t5808: F, t22532: F, t2327: F, t5876: F, t2319: F, t5883: F, t4241: F, t21661: F, t602: F, t2246: F, t5812: F, t10309: F, t13269: F, t13272: F, t13286: F, t13289: F, t13420: F, t1497: F, t21663: F, t21809: F, t2242: F, t2247: F, t2248: F, t2315: F, t4173: F, t4178: F, t5872: F, t60221: F, t60248: F, t644: F, t4186: F, t30: F, t33: F, t49887: F, zeta_threshold: F, t10389: F, t10398: F, t13312: F, t13368: F, t13378: F, t13396: F, t18281: F, t21784: F, t21789: F, t21794: F, t21799: F, t2251: F, t2258: F, t2299: F, t2306: F, t4227: F, t4232: F, t46001: F, t46014: F, t5819: F, t5825: F, t606: F, t633: F, t637: F, t13335: F, t13343: F, t13346: F, t13389: F, t1487: F, t1494: F, t21690: F, t21769: F, t21805: F, t2291: F, t2292: F, t2312: F, t4218: F, t4238: F, t5820: F, t5855: F, t5869: F, t628: F, t641: F, t70: F, t71: F, t77: F, t85: F, t1469: F, t627: F, t72: F, t13406: F, t13409: F, t13414: F, t1471: F, t21686: F, t21687: F, t2252: F, t2259: F, t2260: F, t2263: F, t4188: F, t4191: F, t4196: F, t5854: F, t608: F, t6977: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t60616, t60618, t60620, t60624, t60629, t60650, t60656) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3200::<F>(t1464, t5789, t18177, t575, t1921, t1913, t5808, t22532, t2327, t5876, t2319, t5883);
        let t60692 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3201::<F>(t4241, t21661, t602, t2246, t5812, t10309, t13269, t13272, t13286, t13289, t13420, t1497, t21663, t21809, t2242, t2247, t2248, t2315, t4173, t4178, t5872, t60221, t60248, t644);
        let t60717 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3202::<F>(t4186);
        let t60754 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3203::<F>(t30, t33, t49887, zeta_threshold);
        let t60778 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3204::<F>(t10389, t10398, t13312, t13368, t13378, t13396, t18281, t21784, t21789, t21794, t21799, t2251, t2258, t2299, t2306, t4227, t4232, t46001, t46014, t5819, t5825, t606, t60717, t60754, t633, t637);
        let t60793 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3205::<F>(t13335, t13343, t13346, t13389, t1487, t1494, t21690, t21769, t21805, t2291, t2292, t2312, t4218, t4238, t5819, t5820, t5855, t5869, t60717, t60778, t628, t641, t70, t71, t77, t85);
        let t60829 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3206::<F>(t1469, t627, t72, t13389, t13406, t13409, t13414, t1471, t1494, t21686, t21687, t21805, t2251, t2252, t2259, t2260, t2263, t4186, t4188, t4191, t4196, t4238, t5854, t5869, t608, t6977, t85);
    (t60616, t60618, t60620, t60624, t60629, t60650, t60656, t60692, t60717, t60754, t60793, t60829)
}
