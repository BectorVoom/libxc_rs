//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta720 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2334;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2335;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2336;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2337;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2338;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2339;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2340;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2341;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2342;
use chunk9::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2343;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta720(t20949: f64, t2697: f64, t20882: f64, t9638: f64, t13258: f64, t20988: f64, t13251: f64, t16853: f64, t16946: f64, t16949: f64, t16976: f64, t17013: f64, t2643: f64, t2645: f64, t41467: f64, t4172: f64, t4248: f64, t4257: f64, t46550: f64, t46628: f64, t5591: f64, t58461: f64, t58472: f64, t58474: f64, t58495: f64, t9642: f64, t20887: f64, t13242: f64, t13254: f64, t16839: f64, t16903: f64, t16935: f64, t20972: f64, t20974: f64, t20983: f64, t20986: f64, t2632: f64, t4119: f64, t4178: f64, t4180: f64, t58480: f64, t58482: f64, t58504: f64, t58528: f64, t67607: f64, t9627: f64, t9646: f64, t20969: f64, t2639: f64, t16752: f64, t120: f64, t13222: f64, t13228: f64, t13262: f64, t13350: f64, t13351: f64, t1512: f64, t16836: f64, t16918: f64, t16932: f64, t16937: f64, t17017: f64, t20756: f64, t41453: f64, t4181: f64, t4255: f64, t46574: f64, t5612: f64, t58557: f64, t58765: f64, t67578: f64, t829: f64, t1484: f64, t4233: f64, t5544: f64, t828: f64, t1510: f64, t16944: f64, t2618: f64, t46577: f64, t5585: f64, t5611: f64, t58550: f64, t58569: f64, t58574: f64, t67568: f64, t776: f64, t817: f64, t819: f64, t820: f64, t13278: f64, t5619: f64, t59281: f64, t16662: f64, t16872: f64, t16951: f64, t20800: f64, t20904: f64, t20953: f64, t2623: f64, t2701: f64, t41344: f64, t4236: f64, t46650: f64, t46878: f64, t5527: f64, t5587: f64, t58576: f64, t843: f64, t9607: f64, t67441: f64, t816: f64, t20978: f64, t20938: f64, t838: f64, t13177: f64, t16859: f64, t16912: f64, t20963: f64, t4167: f64, t46692: f64, t47285: f64, t5614: f64, t58616: f64, t58668: f64, t58670: f64, t58853: f64, t831: f64, t9967: f64, t20994: f64, t2563: f64, t16816: f64, t16845: f64, t16893: f64, t16969: f64, t20908: f64, t4182: f64, t46875: f64, t46876: f64, t58705: f64, t58709: f64, t58723: f64, t58731: f64, t58735: f64, t20944: f64, t41011: f64, t119: f64, t13365: f64, t1516: f64, t20943: f64, t210: f64, t2571: f64, t41084: f64, t41161: f64, t4158: f64, t4261: f64, t46887: f64, t46912: f64, t46929: f64, t5567: f64, t5624: f64, t58744: f64, t58834: f64, t67282: f64, t787: f64, t847: f64, t9559: f64, t9667: f64, t46881: f64, t16888: f64, t20947: f64, t20993: f64, t2647: f64, t4240: f64, t46952: f64, t46954: f64, t58642: f64, t58688: f64, t58759: f64, t58761: f64, t58763: f64, t67620: f64, t13005: f64, t13223: f64, t16907: f64, t16985: f64, t20885: f64, t221: f64, t41096: f64, t4191: f64, t5617: f64, t5628: f64, t58791: f64, t58797: f64, t58809: f64, t58845: f64, t58847: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t67696 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2334(t20949, t2697, t20882, t9638, t13258, t20988, t13251, t16853, t16946, t16949, t16976, t17013, t2643, t2645, t41467, t4172, t4248, t4257, t46550, t46628, t5591, t58461, t58472, t58474, t58495, t9642);
        let t67732 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2335(t20887, t9638, t13242, t13251, t13254, t16839, t16903, t16935, t20972, t20974, t20983, t20986, t20988, t2632, t2643, t2645, t4119, t4178, t4180, t58480, t58482, t58504, t58528, t67607, t9627, t9642, t9646);
        let (t67739, t67777) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2336(t20969, t2639, t16752, t2632, t120, t13222, t13228, t13251, t13262, t13350, t13351, t1512, t16836, t16839, t16918, t16932, t16937, t17017, t20756, t20986, t2643, t2645, t41453, t41467, t4178, t4180, t4181, t4255, t46574, t5612, t58557, t58765, t67578, t67607, t829);
        let t67826 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2337(t1484, t4233, t5544, t828, t13222, t13228, t13350, t13351, t1510, t16944, t16949, t20969, t2618, t2643, t4178, t4255, t46577, t5585, t5591, t5611, t58550, t58569, t58574, t67568, t776, t817, t819, t820);
        let t67865 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2338(t13278, t5619, t1512, t59281, t1484, t16662, t16872, t16951, t20800, t20904, t20949, t20953, t2618, t2623, t2701, t4119, t41344, t4172, t4236, t46650, t46878, t5527, t5544, t5587, t58576, t776, t820, t843, t9607);
        let t67898 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2339(t67441, t816, t20978, t9638, t20938, t838, t20953, t2639, t13177, t13222, t13262, t13351, t16839, t16859, t16912, t20963, t2643, t2645, t4167, t46692, t47285, t5614, t58569, t58616, t58668, t58670, t58853, t831, t9642, t9967);
        let t67926 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2340(t20994, t2563, t13251, t13262, t16816, t16836, t16845, t16893, t16969, t20908, t2623, t4178, t4180, t4182, t46875, t46876, t58705, t58709, t58723, t58731, t58735, t67607);
        let t67957 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2341(t20944, t41011, t119, t13365, t1516, t16976, t20943, t210, t2571, t41084, t41161, t4119, t4158, t4261, t46887, t46912, t46929, t5544, t5567, t5624, t58744, t58834, t67282, t776, t787, t820, t843, t847, t9559);
        let t67988 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2342(t13278, t5614, t20963, t9667, t46881, t5587, t13222, t13251, t13350, t16888, t20947, t20993, t210, t2571, t2643, t2645, t2647, t4240, t46952, t46954, t5591, t58642, t58688, t58759, t58761, t58763, t67620, t776, t829);
        let (t68010, t68018) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2343(t20947, t776, t13005, t13222, t13223, t13251, t13350, t13365, t16907, t16985, t20885, t20972, t221, t2643, t41096, t4172, t4191, t4255, t5617, t5628, t58642, t58791, t58797, t58809, t58845, t58847);
    (t67696, t67732, t67739, t67777, t67826, t67865, t67898, t67926, t67957, t67988, t68010, t68018)
}
