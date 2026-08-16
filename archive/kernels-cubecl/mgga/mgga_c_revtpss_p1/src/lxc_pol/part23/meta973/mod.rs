//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta973 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3298;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3299;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3300;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3301;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3302;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3303;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3304;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3305;
use chunk8::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3306;
use chunk9::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3307;
use chunk10::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3308;
use chunk11::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3309;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta973<F: Float>(t2782: F, t4086: F, t543: F, t86455: F, t86470: F, t14192: F, t86445: F, t9994: F, t22964: F, t545: F, t689: F, t869: F, t86506: F, t1399: F, t14255: F, t21981: F, t21990: F, t47417: F, t47442: F, t49276: F, t49361: F, t5745: F, t5755: F, t6862: F, t6874: F, t75252: F, t820: F, t86441: F, t4003: F, t5744: F, t22912: F, t4101: F, t686: F, t72: F, t85659: F, t4100: F, t14193: F, t14224: F, t22005: F, t22009: F, t47444: F, t5675: F, t75269: F, t75274: F, t85580: F, t22253: F, t47450: F, t47454: F, t47455: F, t49426: F, t49429: F, t49432: F, t5767: F, t75298: F, t75302: F, t75307: F, t1904: F, t22445: F, t14127: F, t1424: F, t1427: F, t14299: F, t1444: F, t213: F, t22387: F, t22395: F, t22974: F, t23037: F, t4118: F, t46362: F, t46392: F, t46412: F, t47389: F, t47395: F, t47574: F, t47591: F, t48029: F, t48036: F, t48040: F, t48042: F, t49172: F, t49177: F, t49178: F, t49187: F, t49190: F, t49322: F, t49354: F, t546: F, t5659: F, t5715: F, t6919: F, t74794: F, t74797: F, t74807: F, t74810: F, t74813: F, t74824: F, t74826: F, t74862: F, t74866: F, t74873: F, t74880: F, t74884: F, t74999: F, t75003: F, t75005: F, t75014: F, t75018: F, t75089: F, t75092: F, t75215: F, t75219: F, t86280: F, t86346: F, t86350: F, t86354: F, t86358: F, t86387: F, t86405: F, t86422: F, t86453: F, t86474: F, t86498: F, t86533: F, t86556: F, t86567: F, t47603: F, t13729: F, t556: F, t6918: F, t1445: F, t22390: F, t22414: F, t22975: F, t4071: F, t47601: F, t47618: F, t47793: F, t47794: F, t49513: F, t5775: F, t74829: F, t74836: F, t74838: F, t74843: F, t74849: F, t74853: F, t75336: F, t1343: F, t13600: F, t1450: F, t1868: F, t198: F, t22466: F, t22486: F, t39419: F, t39422: F, t4139: F, t46297: F, t46963: F, t46970: F, t47753: F, t47760: F, t48157: F, t48159: F, t532: F, t5536: F, t5591: F, t6836: F, t75379: F, t85390: F, t85391: F, t85442: F, t85466: F, t85482: F, t85498: F, t85887: F, t85888: F, t85889: F, t86291: F, t86308: F, t86340: F, t23059: F, t39528: F, t39531: F, t48234: F, t48236: F, t48241: F, t48244: F, t75389: F, t85896: F, t85897: F, t85898: F, t85899: F, t22483: F, t22809: F, t39773: F, t4140: F, t46996: F, t46998: F, t47003: F, t48256: F, t48259: F, t48261: F, t5541: F, t5778: F, t85905: F, t85906: F, t1448: F, t39799: F, t39807: F, t39813: F, t47059: F, t48271: F, t5627: F, t6816: F, t85913: F, t85914: F, t85918: F, t85919: F, t22852: F, t47076: F, t48291: F, t48293: F, t85923: F, t85924: F, t85925: F, t85926: F, t85928: F, t85930: F, t85932: F) -> (F, F, F, F, F, F, F, F) {
        let (t86575, t86582, t86586, t86597) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3298::<F>(t2782, t4086, t543, t86455, t86470, t14192, t86445, t9994, t22964, t545, t689, t869);
        let t86616 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3299::<F>(t2782, t4086, t543, t86506, t86445, t1399, t14255, t21981, t21990, t47417, t47442, t49276, t49361, t5745, t5755, t6862, t6874, t75252, t820, t86441, t86597);
        let (t86634, t86639, t86643, t86647) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3300::<F>(t2782, t4003, t5744, t86470, t22912, t4101, t686, t72, t543, t85659, t4100, t86445);
        let t86649 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3301::<F>(t1399, t14193, t14224, t22005, t22009, t47444, t5675, t5745, t5755, t75269, t75274, t85580, t86445, t86506, t86634, t86639, t86643, t86647);
        let t86665 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3302::<F>(t2782, t4086, t543, t86441, t22253, t47450, t47454, t47455, t49426, t49429, t49432, t5767, t75298, t75302, t75307, t820);
        let t86691 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3303::<F>(t1904, t22445, t689, t14127, t1424, t1427, t14299, t1444, t213, t22005, t22387, t22395, t22912, t22974, t23037, t4118, t46362, t46392, t46412, t47389, t47395, t47574, t47591, t48029, t48036, t48040, t48042, t49172, t49177, t49178, t49187, t49190, t49322, t49354, t546, t5659, t5675, t5715, t5745, t5755, t6919, t74794, t74797, t74807, t74810, t74813, t74824, t74826, t74862, t74866, t74873, t74880, t74884, t74999, t75003, t75005, t75014, t75018, t75089, t75092, t75215, t75219, t820, t86280, t86346, t86350, t86354, t86358, t86387, t86405, t86422, t86453, t86474, t86498, t86506, t86533, t86556, t86567, t86575, t86582, t86586, t86616, t86649, t86665);
        let t86718 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3304::<F>(t22974, t47603, t686, t72, t213, t22964, t13729, t2782, t556, t6918, t1445, t22390, t22414, t22975, t4071, t47601, t47618, t47793, t47794, t49513, t5775, t74829, t74836, t74838, t74843, t74849, t74853, t75336);
        let t86728 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3305::<F>(t1343, t13600, t1450, t1868, t198, t22466, t22486, t39419, t39422, t4139, t46297, t46963, t46970, t47753, t47760, t48157, t48159, t532, t5536, t5591, t6836, t75379, t85390, t85391, t85442, t85466, t85482, t85498, t85887, t85888, t85889, t86291, t86308, t86340, t86691, t86718);
        let (t86731, t86741) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3306::<F>(t1450, t23059, t1868, t39528, t39531, t4139, t48234, t48236, t48241, t48244, t75389, t85896, t85897, t85898, t85899);
        let t86751 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3307::<F>(t22483, t22809, t39773, t4139, t4140, t46996, t46998, t47003, t48256, t48259, t48261, t5541, t5778, t85905, t85906);
        let (t86753, t86764) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3308::<F>(t1448, t6836, t13600, t22466, t39799, t39807, t39813, t4139, t47059, t48271, t5536, t5627, t6816, t85913, t85914, t85918, t85919);
        let (t86771, t86782) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3309::<F>(t1448, t6816, t22852, t4140, t47076, t48291, t48293, t5536, t85923, t85924, t85925, t85926, t85928, t85930, t85932);
    (t86728, t86731, t86741, t86751, t86753, t86764, t86771, t86782)
}
