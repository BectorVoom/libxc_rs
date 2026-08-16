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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta721<F: Float>(t20908: F, t2697: F, t1509: F, t5611: F, t13222: F, t13251: F, t16914: F, t16924: F, t17009: F, t20896: F, t2623: F, t2643: F, t2647: F, t46692: F, t47044: F, t47047: F, t5593: F, t58859: F, t58873: F, t58885: F, t58890: F, t58900: F, t829: F, t13012: F, t20927: F, t13005: F, t41144: F, t41155: F, t41156: F, t41185: F, t41190: F, t46764: F, t46769: F, t46838: F, t59138: F, t59140: F, t68010: F, t12988: F, t16771: F, t20756: F, t20800: F, t213: F, t221: F, t4119: F, t41200: F, t4127: F, t46770: F, t46772: F, t46783: F, t46847: F, t5544: F, t59154: F, t59156: F, t59165: F, t59173: F, t776: F, t12984: F, t12998: F, t686: F, t20933: F, t2563: F, t20923: F, t41011: F, t118: F, t41170: F, t794: F, t16662: F, t4128: F, t46790: F, t46794: F, t46796: F, t46806: F, t46856: F, t59195: F, t2576: F, t210: F, t214: F, t41209: F, t41212: F, t41217: F, t59204: F, t59206: F, t59214: F, t59216: F, t59218: F, t59221: F, t59224: F, t67282: F, t787: F, t225: F, t21008: F, t9573: F, t13228: F, t1495: F, t1510: F, t16836: F, t16851: F, t16928: F, t237: F, t249: F, t2571: F, t41130: F, t41139: F, t41363: F, t4178: F, t47039: F, t47080: F, t47094: F, t47231: F, t47270: F, t58569: F, t59100: F, t13242: F, t16891: F, t16912: F, t20885: F, t20887: F, t20891: F, t232: F, t2645: F, t4180: F, t4181: F, t4234: F, t47277: F, t58495: F, t59251: F, t59255: F, t59257: F, t59259: F, t59261: F, t67607: F, t9642: F, t13360: F, t5624: F, t1516: F, t58844: F, t5628: F, t67441: F, t842: F, t59263: F, t59276: F, t59279: F, t59282: F, t59288: F, t59298: F, t59308: F, t59310: F, t59322: F, t849: F, t67636: F, t67667: F, t67696: F, t67732: F, t67777: F, t67826: F, t67865: F, t67898: F, t67926: F, t67957: F, t67988: F, t68018: F, t5631: F, t9975: F, t13380: F, t13397: F, t1523: F, t16673: F, t16811: F, t17030: F, t20876: F, t20986: F, t25115: F, t2617: F, t4166: F, t4281: F, t4282: F, t4286: F, t4291: F, t58181: F, t58262: F, t59331: F, t67739: F, t828: F, t10054: F, t1499: F, t1525: F, t16754: F, t16805: F, t17023: F, t20853: F, t20854: F, t20857: F, t20858: F, t20861: F, t20862: F, t20937: F, t2732: F, t40917: F, t4298: F, t5575: F, t812: F, t863: F) -> (F, F, F, F, F, F) {
        let t68048 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2344::<F>(t20908, t2697, t1509, t5611, t13222, t13251, t16914, t16924, t17009, t20896, t2623, t2643, t2647, t46692, t47044, t47047, t5593, t58859, t58873, t58885, t58890, t58900, t829);
        let t68077 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2345::<F>(t13012, t20927, t13005, t41144, t41155, t41156, t41185, t41190, t46764, t46769, t46838, t59138, t59140, t68010);
        let t68102 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2346::<F>(t12988, t13005, t16771, t20756, t20800, t213, t221, t4119, t41200, t4127, t46770, t46772, t46783, t46847, t5544, t59154, t59156, t59165, t59173, t776);
        let (t68110, t68116, t68118, t68122) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2347::<F>(t12984, t12998, t5544, t686, t20933, t2563, t20923, t41011, t118, t20756, t41170, t794);
        let t68124 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2348::<F>(t16662, t221, t4127, t4128, t46790, t46794, t46796, t46806, t46856, t59195, t68110, t68116, t68118, t68122);
        let t68141 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2349::<F>(t118, t20800, t2576, t794, t210, t214, t41209, t41212, t41217, t59204, t59206, t59214, t59216, t59218, t59221, t59224, t67282, t787);
        let (t68143, t68144, t68150) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2350::<F>(t68077, t68102, t68124, t68141, t225, t21008, t9573, t13228, t1495, t1510, t16662, t16836, t16851, t16928, t210, t237, t249, t2571, t2643, t41130, t41139, t41363, t4178, t46692, t47039, t47080, t47094, t47231, t47270, t58569, t59100);
        let t68186 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2351::<F>(t13242, t1510, t16662, t16891, t16912, t20885, t20887, t20891, t232, t2643, t2645, t2647, t4180, t4181, t4234, t47277, t58495, t59251, t59255, t59257, t59259, t59261, t67607, t9642);
        let t68207 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2352::<F>(t20896, t2697, t13360, t5624, t1516, t58844, t5628, t67441, t842, t59263, t59276, t59279, t59282, t59288, t59298, t59308, t59310, t59322, t849);
        let t68211 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2353::<F>(t67636, t67667, t67696, t67732, t67777, t67826, t67865, t67898, t67926, t67957, t67988, t68018, t68048, t68150, t68186, t68207);
        let (t68217, t68256) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2354::<F>(t1509, t5631, t5611, t9975, t13380, t13397, t1510, t1523, t16673, t16811, t17030, t20876, t20986, t25115, t2617, t4166, t4281, t4282, t4286, t4291, t58181, t58262, t59331, t67739, t828, t829);
        let t68299 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2355::<F>(t10054, t1499, t1525, t16754, t16805, t17023, t20853, t20854, t20857, t20858, t20861, t20862, t20937, t2617, t2732, t40917, t4166, t4298, t5575, t812, t863);
    (t68143, t68144, t68211, t68217, t68256, t68299)
}
