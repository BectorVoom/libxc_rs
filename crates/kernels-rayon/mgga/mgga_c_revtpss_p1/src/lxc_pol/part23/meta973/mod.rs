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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta973(t2782: f64, t4086: f64, t543: f64, t86455: f64, t86470: f64, t14192: f64, t86445: f64, t9994: f64, t22964: f64, t545: f64, t689: f64, t869: f64, t86506: f64, t1399: f64, t14255: f64, t21981: f64, t21990: f64, t47417: f64, t47442: f64, t49276: f64, t49361: f64, t5745: f64, t5755: f64, t6862: f64, t6874: f64, t75252: f64, t820: f64, t86441: f64, t4003: f64, t5744: f64, t22912: f64, t4101: f64, t686: f64, t72: f64, t85659: f64, t4100: f64, t14193: f64, t14224: f64, t22005: f64, t22009: f64, t47444: f64, t5675: f64, t75269: f64, t75274: f64, t85580: f64, t22253: f64, t47450: f64, t47454: f64, t47455: f64, t49426: f64, t49429: f64, t49432: f64, t5767: f64, t75298: f64, t75302: f64, t75307: f64, t1904: f64, t22445: f64, t14127: f64, t1424: f64, t1427: f64, t14299: f64, t1444: f64, t213: f64, t22387: f64, t22395: f64, t22974: f64, t23037: f64, t4118: f64, t46362: f64, t46392: f64, t46412: f64, t47389: f64, t47395: f64, t47574: f64, t47591: f64, t48029: f64, t48036: f64, t48040: f64, t48042: f64, t49172: f64, t49177: f64, t49178: f64, t49187: f64, t49190: f64, t49322: f64, t49354: f64, t546: f64, t5659: f64, t5715: f64, t6919: f64, t74794: f64, t74797: f64, t74807: f64, t74810: f64, t74813: f64, t74824: f64, t74826: f64, t74862: f64, t74866: f64, t74873: f64, t74880: f64, t74884: f64, t74999: f64, t75003: f64, t75005: f64, t75014: f64, t75018: f64, t75089: f64, t75092: f64, t75215: f64, t75219: f64, t86280: f64, t86346: f64, t86350: f64, t86354: f64, t86358: f64, t86387: f64, t86405: f64, t86422: f64, t86453: f64, t86474: f64, t86498: f64, t86533: f64, t86556: f64, t86567: f64, t47603: f64, t13729: f64, t556: f64, t6918: f64, t1445: f64, t22390: f64, t22414: f64, t22975: f64, t4071: f64, t47601: f64, t47618: f64, t47793: f64, t47794: f64, t49513: f64, t5775: f64, t74829: f64, t74836: f64, t74838: f64, t74843: f64, t74849: f64, t74853: f64, t75336: f64, t1343: f64, t13600: f64, t1450: f64, t1868: f64, t198: f64, t22466: f64, t22486: f64, t39419: f64, t39422: f64, t4139: f64, t46297: f64, t46963: f64, t46970: f64, t47753: f64, t47760: f64, t48157: f64, t48159: f64, t532: f64, t5536: f64, t5591: f64, t6836: f64, t75379: f64, t85390: f64, t85391: f64, t85442: f64, t85466: f64, t85482: f64, t85498: f64, t85887: f64, t85888: f64, t85889: f64, t86291: f64, t86308: f64, t86340: f64, t23059: f64, t39528: f64, t39531: f64, t48234: f64, t48236: f64, t48241: f64, t48244: f64, t75389: f64, t85896: f64, t85897: f64, t85898: f64, t85899: f64, t22483: f64, t22809: f64, t39773: f64, t4140: f64, t46996: f64, t46998: f64, t47003: f64, t48256: f64, t48259: f64, t48261: f64, t5541: f64, t5778: f64, t85905: f64, t85906: f64, t1448: f64, t39799: f64, t39807: f64, t39813: f64, t47059: f64, t48271: f64, t5627: f64, t6816: f64, t85913: f64, t85914: f64, t85918: f64, t85919: f64, t22852: f64, t47076: f64, t48291: f64, t48293: f64, t85923: f64, t85924: f64, t85925: f64, t85926: f64, t85928: f64, t85930: f64, t85932: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t86575, t86582, t86586, t86597) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3298(t2782, t4086, t543, t86455, t86470, t14192, t86445, t9994, t22964, t545, t689, t869);
        let t86616 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3299(t2782, t4086, t543, t86506, t86445, t1399, t14255, t21981, t21990, t47417, t47442, t49276, t49361, t5745, t5755, t6862, t6874, t75252, t820, t86441, t86597);
        let (t86634, t86639, t86643, t86647) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3300(t2782, t4003, t5744, t86470, t22912, t4101, t686, t72, t543, t85659, t4100, t86445);
        let t86649 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3301(t1399, t14193, t14224, t22005, t22009, t47444, t5675, t5745, t5755, t75269, t75274, t85580, t86445, t86506, t86634, t86639, t86643, t86647);
        let t86665 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3302(t2782, t4086, t543, t86441, t22253, t47450, t47454, t47455, t49426, t49429, t49432, t5767, t75298, t75302, t75307, t820);
        let t86691 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3303(t1904, t22445, t689, t14127, t1424, t1427, t14299, t1444, t213, t22005, t22387, t22395, t22912, t22974, t23037, t4118, t46362, t46392, t46412, t47389, t47395, t47574, t47591, t48029, t48036, t48040, t48042, t49172, t49177, t49178, t49187, t49190, t49322, t49354, t546, t5659, t5675, t5715, t5745, t5755, t6919, t74794, t74797, t74807, t74810, t74813, t74824, t74826, t74862, t74866, t74873, t74880, t74884, t74999, t75003, t75005, t75014, t75018, t75089, t75092, t75215, t75219, t820, t86280, t86346, t86350, t86354, t86358, t86387, t86405, t86422, t86453, t86474, t86498, t86506, t86533, t86556, t86567, t86575, t86582, t86586, t86616, t86649, t86665);
        let t86718 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3304(t22974, t47603, t686, t72, t213, t22964, t13729, t2782, t556, t6918, t1445, t22390, t22414, t22975, t4071, t47601, t47618, t47793, t47794, t49513, t5775, t74829, t74836, t74838, t74843, t74849, t74853, t75336);
        let t86728 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3305(t1343, t13600, t1450, t1868, t198, t22466, t22486, t39419, t39422, t4139, t46297, t46963, t46970, t47753, t47760, t48157, t48159, t532, t5536, t5591, t6836, t75379, t85390, t85391, t85442, t85466, t85482, t85498, t85887, t85888, t85889, t86291, t86308, t86340, t86691, t86718);
        let (t86731, t86741) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3306(t1450, t23059, t1868, t39528, t39531, t4139, t48234, t48236, t48241, t48244, t75389, t85896, t85897, t85898, t85899);
        let t86751 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3307(t22483, t22809, t39773, t4139, t4140, t46996, t46998, t47003, t48256, t48259, t48261, t5541, t5778, t85905, t85906);
        let (t86753, t86764) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3308(t1448, t6836, t13600, t22466, t39799, t39807, t39813, t4139, t47059, t48271, t5536, t5627, t6816, t85913, t85914, t85918, t85919);
        let (t86771, t86782) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3309(t1448, t6816, t22852, t4140, t47076, t48291, t48293, t5536, t85923, t85924, t85925, t85926, t85928, t85930, t85932);
    (t86728, t86731, t86741, t86751, t86753, t86764, t86771, t86782)
}
