//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta956 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3200;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3201;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3202;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3203;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3204;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3205;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3206;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta956(t1464: f64, t5789: f64, t18177: f64, t575: f64, t1921: f64, t1913: f64, t5808: f64, t22532: f64, t2327: f64, t5876: f64, t2319: f64, t5883: f64, t4241: f64, t21661: f64, t602: f64, t2246: f64, t5812: f64, t10309: f64, t13269: f64, t13272: f64, t13286: f64, t13289: f64, t13420: f64, t1497: f64, t21663: f64, t21809: f64, t2242: f64, t2247: f64, t2248: f64, t2315: f64, t4173: f64, t4178: f64, t5872: f64, t60221: f64, t60248: f64, t644: f64, t4186: f64, t30: f64, t33: f64, t49887: f64, zeta_threshold: f64, t10389: f64, t10398: f64, t13312: f64, t13368: f64, t13378: f64, t13396: f64, t18281: f64, t21784: f64, t21789: f64, t21794: f64, t21799: f64, t2251: f64, t2258: f64, t2299: f64, t2306: f64, t4227: f64, t4232: f64, t46001: f64, t46014: f64, t5819: f64, t5825: f64, t606: f64, t633: f64, t637: f64, t13335: f64, t13343: f64, t13346: f64, t13389: f64, t1487: f64, t1494: f64, t21690: f64, t21769: f64, t21805: f64, t2291: f64, t2292: f64, t2312: f64, t4218: f64, t4238: f64, t5820: f64, t5855: f64, t5869: f64, t628: f64, t641: f64, t70: f64, t71: f64, t77: f64, t85: f64, t1469: f64, t627: f64, t72: f64, t13406: f64, t13409: f64, t13414: f64, t1471: f64, t21686: f64, t21687: f64, t2252: f64, t2259: f64, t2260: f64, t2263: f64, t4188: f64, t4191: f64, t4196: f64, t5854: f64, t608: f64, t6977: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t60616, t60618, t60620, t60624, t60629, t60650, t60656) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3200(t1464, t5789, t18177, t575, t1921, t1913, t5808, t22532, t2327, t5876, t2319, t5883);
        let t60692 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3201(t4241, t21661, t602, t2246, t5812, t10309, t13269, t13272, t13286, t13289, t13420, t1497, t21663, t21809, t2242, t2247, t2248, t2315, t4173, t4178, t5872, t60221, t60248, t644);
        let t60717 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3202(t4186);
        let t60754 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3203(t30, t33, t49887, zeta_threshold);
        let t60778 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3204(t10389, t10398, t13312, t13368, t13378, t13396, t18281, t21784, t21789, t21794, t21799, t2251, t2258, t2299, t2306, t4227, t4232, t46001, t46014, t5819, t5825, t606, t60717, t60754, t633, t637);
        let t60793 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3205(t13335, t13343, t13346, t13389, t1487, t1494, t21690, t21769, t21805, t2291, t2292, t2312, t4218, t4238, t5819, t5820, t5855, t5869, t60717, t60778, t628, t641, t70, t71, t77, t85);
        let t60829 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3206(t1469, t627, t72, t13389, t13406, t13409, t13414, t1471, t1494, t21686, t21687, t21805, t2251, t2252, t2259, t2260, t2263, t4186, t4188, t4191, t4196, t4238, t5854, t5869, t608, t6977, t85);
    (t60616, t60618, t60620, t60624, t60629, t60650, t60656, t60692, t60717, t60754, t60793, t60829)
}
