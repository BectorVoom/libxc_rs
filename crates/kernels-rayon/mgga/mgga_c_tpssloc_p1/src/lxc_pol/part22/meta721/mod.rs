//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta721 (260520-c91 hierarchical CSE).
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
mod chunk10;
mod chunk11;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2344;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2345;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2346;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2347;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2348;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2349;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2350;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2351;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2352;
use chunk9::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2353;
use chunk10::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2354;
use chunk11::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2355;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta721(t20908: f64, t2697: f64, t1509: f64, t5611: f64, t13222: f64, t13251: f64, t16914: f64, t16924: f64, t17009: f64, t20896: f64, t2623: f64, t2643: f64, t2647: f64, t46692: f64, t47044: f64, t47047: f64, t5593: f64, t58859: f64, t58873: f64, t58885: f64, t58890: f64, t58900: f64, t829: f64, t13012: f64, t20927: f64, t13005: f64, t41144: f64, t41155: f64, t41156: f64, t41185: f64, t41190: f64, t46764: f64, t46769: f64, t46838: f64, t59138: f64, t59140: f64, t68010: f64, t12988: f64, t16771: f64, t20756: f64, t20800: f64, t213: f64, t221: f64, t4119: f64, t41200: f64, t4127: f64, t46770: f64, t46772: f64, t46783: f64, t46847: f64, t5544: f64, t59154: f64, t59156: f64, t59165: f64, t59173: f64, t776: f64, t12984: f64, t12998: f64, t686: f64, t20933: f64, t2563: f64, t20923: f64, t41011: f64, t118: f64, t41170: f64, t794: f64, t16662: f64, t4128: f64, t46790: f64, t46794: f64, t46796: f64, t46806: f64, t46856: f64, t59195: f64, t2576: f64, t210: f64, t214: f64, t41209: f64, t41212: f64, t41217: f64, t59204: f64, t59206: f64, t59214: f64, t59216: f64, t59218: f64, t59221: f64, t59224: f64, t67282: f64, t787: f64, t225: f64, t21008: f64, t9573: f64, t13228: f64, t1495: f64, t1510: f64, t16836: f64, t16851: f64, t16928: f64, t237: f64, t249: f64, t2571: f64, t41130: f64, t41139: f64, t41363: f64, t4178: f64, t47039: f64, t47080: f64, t47094: f64, t47231: f64, t47270: f64, t58569: f64, t59100: f64, t13242: f64, t16891: f64, t16912: f64, t20885: f64, t20887: f64, t20891: f64, t232: f64, t2645: f64, t4180: f64, t4181: f64, t4234: f64, t47277: f64, t58495: f64, t59251: f64, t59255: f64, t59257: f64, t59259: f64, t59261: f64, t67607: f64, t9642: f64, t13360: f64, t5624: f64, t1516: f64, t58844: f64, t5628: f64, t67441: f64, t842: f64, t59263: f64, t59276: f64, t59279: f64, t59282: f64, t59288: f64, t59298: f64, t59308: f64, t59310: f64, t59322: f64, t849: f64, t67636: f64, t67667: f64, t67696: f64, t67732: f64, t67777: f64, t67826: f64, t67865: f64, t67898: f64, t67926: f64, t67957: f64, t67988: f64, t68018: f64, t5631: f64, t9975: f64, t13380: f64, t13397: f64, t1523: f64, t16673: f64, t16811: f64, t17030: f64, t20876: f64, t20986: f64, t25115: f64, t2617: f64, t4166: f64, t4281: f64, t4282: f64, t4286: f64, t4291: f64, t58181: f64, t58262: f64, t59331: f64, t67739: f64, t828: f64, t10054: f64, t1499: f64, t1525: f64, t16754: f64, t16805: f64, t17023: f64, t20853: f64, t20854: f64, t20857: f64, t20858: f64, t20861: f64, t20862: f64, t20937: f64, t2732: f64, t40917: f64, t4298: f64, t5575: f64, t812: f64, t863: f64) -> (f64, f64, f64, f64, f64, f64) {
        let t68048 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2344(t20908, t2697, t1509, t5611, t13222, t13251, t16914, t16924, t17009, t20896, t2623, t2643, t2647, t46692, t47044, t47047, t5593, t58859, t58873, t58885, t58890, t58900, t829);
        let t68077 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2345(t13012, t20927, t13005, t41144, t41155, t41156, t41185, t41190, t46764, t46769, t46838, t59138, t59140, t68010);
        let t68102 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2346(t12988, t13005, t16771, t20756, t20800, t213, t221, t4119, t41200, t4127, t46770, t46772, t46783, t46847, t5544, t59154, t59156, t59165, t59173, t776);
        let (t68110, t68116, t68118, t68122) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2347(t12984, t12998, t5544, t686, t20933, t2563, t20923, t41011, t118, t20756, t41170, t794);
        let t68124 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2348(t16662, t221, t4127, t4128, t46790, t46794, t46796, t46806, t46856, t59195, t68110, t68116, t68118, t68122);
        let t68141 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2349(t118, t20800, t2576, t794, t210, t214, t41209, t41212, t41217, t59204, t59206, t59214, t59216, t59218, t59221, t59224, t67282, t787);
        let (t68143, t68144, t68150) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2350(t68077, t68102, t68124, t68141, t225, t21008, t9573, t13228, t1495, t1510, t16662, t16836, t16851, t16928, t210, t237, t249, t2571, t2643, t41130, t41139, t41363, t4178, t46692, t47039, t47080, t47094, t47231, t47270, t58569, t59100);
        let t68186 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2351(t13242, t1510, t16662, t16891, t16912, t20885, t20887, t20891, t232, t2643, t2645, t2647, t4180, t4181, t4234, t47277, t58495, t59251, t59255, t59257, t59259, t59261, t67607, t9642);
        let t68207 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2352(t20896, t2697, t13360, t5624, t1516, t58844, t5628, t67441, t842, t59263, t59276, t59279, t59282, t59288, t59298, t59308, t59310, t59322, t849);
        let t68211 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2353(t67636, t67667, t67696, t67732, t67777, t67826, t67865, t67898, t67926, t67957, t67988, t68018, t68048, t68150, t68186, t68207);
        let (t68217, t68256) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2354(t1509, t5631, t5611, t9975, t13380, t13397, t1510, t1523, t16673, t16811, t17030, t20876, t20986, t25115, t2617, t4166, t4281, t4282, t4286, t4291, t58181, t58262, t59331, t67739, t828, t829);
        let t68299 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2355(t10054, t1499, t1525, t16754, t16805, t17023, t20853, t20854, t20857, t20858, t20861, t20862, t20937, t2617, t2732, t40917, t4166, t4298, t5575, t812, t863);
    (t68143, t68144, t68211, t68217, t68256, t68299)
}
