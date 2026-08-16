//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta742 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2452;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2453;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2454;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2455;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2456;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2457;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2458;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2459;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2460;
use chunk9::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2461;
use chunk10::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2462;
use chunk11::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2463;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta742(t17863: f64, t2986: f64, t48279: f64, t10231: f64, t21409: f64, t973: f64, t21462: f64, t2970: f64, t10186: f64, t1597: f64, t17841: f64, t21410: f64, t21419: f64, t21444: f64, t21463: f64, t2960: f64, t343: f64, t4518: f64, t4540: f64, t4546: f64, t48067: f64, t5836: f64, t61288: f64, t61291: f64, t61294: f64, t67060: f64, t68458: f64, t68554: f64, t977: f64, t978: f64, t984: f64, t17635: f64, t4337: f64, t10254: f64, t21510: f64, t13769: f64, t13835: f64, t13839: f64, t1409: f64, t17748: f64, t17800: f64, t17804: f64, t2988: f64, t2989: f64, t4531: f64, t5681: f64, t5685: f64, t61082: f64, t61103: f64, t61279: f64, t61307: f64, t61310: f64, t61313: f64, t61322: f64, t61327: f64, t61365: f64, t6733: f64, t21472: f64, t13822: f64, t21452: f64, t21468: f64, t42972: f64, t21453: f64, t21469: f64, t21473: f64, t48293: f64, t48321: f64, t61383: f64, t61387: f64, t61391: f64, t61394: f64, t61397: f64, t61405: f64, t61408: f64, t61422: f64, t61427: f64, t21456: f64, t28565: f64, t48329: f64, t48336: f64, t48339: f64, t48374: f64, t48379: f64, t48382: f64, t48397: f64, t61447: f64, t61472: f64, t61489: f64, t61495: f64, t61557: f64, t61597: f64, t61600: f64, t61602: f64, t69533: f64, t69574: f64, t69665: f64, t69695: f64, t1049: f64, t1052: f64, t1065: f64, t1625: f64, t1635: f64, t17583: f64, t17588: f64, t17875: f64, t18071: f64, t18166: f64, t21480: f64, t21662: f64, t21663: f64, t3026: f64, t3174: f64, t381: f64, t388: f64, t4557: f64, t4660: f64, t4665: f64, t4694: f64, t61058: f64, t21682: f64, t225: f64, t1009: f64, t1057: f64, t10482: f64, t5866: f64, t1022: f64, t1058: f64, t1060: f64, t1061: f64, t11059: f64, t14618: f64, t18083: f64, t18100: f64, t18111: f64, t18138: f64, t18162: f64, t21594: f64, t21618: f64, t21637: f64, t21643: f64, t23508: f64, t3180: f64, t3186: f64, t360: f64, t43503: f64, t43576: f64, t43577: f64, t4669: f64, t50508: f64, t50509: f64, t5932: f64, t68441: f64, t68706: f64, t68708: f64, t68710: f64, t68715: f64, t68717: f64, t68760: f64, t68762: f64, t68764: f64, t68767: f64, t68769: f64, t68771: f64, t68773: f64, t68775: f64, t68883: f64, t68885: f64, t68887: f64, t68891: f64, t68894: f64, t68896: f64, t68905: f64, t68910: f64, t68912: f64, t68916: f64, t68918: f64, t68920: f64, t68923: f64, t68926: f64, t68930: f64, t68934: f64, t68936: f64, t68938: f64, t68940: f64, t68943: f64, t68947: f64, t68949: f64, t68951: f64, t68954: f64, t68995: f64, t68998: f64, t69003: f64, t69005: f64, t69011: f64, t69014: f64, t69018: f64, t69021: f64, t69023: f64, t69025: f64, t69027: f64, t69030: f64, t69036: f64, t69253: f64, t69255: f64, t69257: f64, t69259: f64, t69261: f64, t69453: f64, t69456: f64, t69459: f64, t69461: f64, t69313: f64, t69335: f64, t69337: f64, t69340: f64, t69343: f64, t69346: f64, t69350: f64, t69353: f64, t69357: f64, t69469: f64, t69471: f64, t69263: f64, t69288: f64, t69291: f64, t69294: f64, t69297: f64, t69299: f64, t69302: f64, t69305: f64, t69307: f64, t69310: f64, t4657: f64, t5872: f64, t1063: f64, t11034: f64, t11060: f64, t1610: f64, t18086: f64, t18089: f64, t18094: f64, t18129: f64, t21481: f64, t21614: f64, t21626: f64, t21657: f64, t3200: f64, t3201: f64, t43553: f64, t43554: f64, t4649: f64, t4673: f64, t4678: f64, t47853: f64, t5928: f64, t21390: f64, t11065: f64, t14608: f64, t1632: f64, t17876: f64, t18103: f64, t18117: f64, t18131: f64, t21617: f64, t21644: f64, t21647: f64, t21650: f64, t3188: f64, t43470: f64, t43473: f64, t4684: f64, t5914: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t69741 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2452(t17863, t2986, t48279, t10231, t21409, t973, t21462, t2970, t10186, t1597, t17841, t21410, t21419, t21444, t21463, t2960, t343, t4518, t4540, t4546, t48067, t5836, t61288, t61291, t61294, t67060, t68458, t68554, t977, t978, t984);
        let (t69742, t69791) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2453(t17635, t4337, t10254, t21510, t13769, t13835, t13839, t1409, t17748, t17800, t17804, t2986, t2988, t2989, t4518, t4531, t4540, t5681, t5685, t61082, t61103, t61279, t61307, t61310, t61313, t61322, t61327, t61365, t6733);
        let t69817 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2454(t21472, t2970, t973, t13822, t21452, t21468, t42972, t21453, t21469, t21473, t2960, t48293, t48321, t61383, t61387, t61391, t61394, t61397, t61405, t61408, t61422, t61427);
        let t69837 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2455(t21456, t28565, t343, t4540, t4546, t48329, t48336, t48339, t48374, t48379, t48382, t48397, t61447, t61472, t61489, t61495, t61557, t61597, t61600, t61602, t973, t984);
        let (t69840, t69860) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2456(t69533, t69574, t69665, t69695, t69741, t69791, t69817, t69837, t1049, t1052, t1065, t1625, t1635, t17583, t17588, t17875, t18071, t18166, t21480, t21662, t21663, t3026, t3174, t381, t388, t4557, t4660, t4665, t4694, t61058);
        let (t69871, t69923, t69935, t69942) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2457(t21682, t225, t1009, t21480, t1057, t10482, t5866, t1022, t1049, t1058, t1060, t1061, t11059, t14618, t18083, t18100, t18111, t18138, t18162, t21594, t21618, t21637, t21643, t23508, t3180, t3186, t360, t43503, t43576, t43577, t4669, t50508, t50509, t5932);
        let t69958 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2458(t68441, t68706, t68708, t68710, t68715, t68717, t68760, t68762, t68764, t68767, t68769, t68771, t68773, t68775, t68883, t68885, t68887, t68891, t68894, t68896);
        let t69959 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2459(t68905, t68910, t68912, t68916, t68918, t68920, t68923, t68926, t68930, t68934, t68936, t68938, t68940, t68943, t68947, t68949, t68951, t68954, t68995, t68998);
        let t69961 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2460(t69003, t69005, t69011, t69014, t69018, t69021, t69023, t69025, t69027, t69030, t69036, t69253, t69255, t69257, t69259, t69261, t69453, t69456, t69459, t69461);
        let t69966 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2461(t69313, t69335, t69337, t69340, t69343, t69346, t69350, t69353, t69357, t69469, t69471, t69263, t69288, t69291, t69294, t69297, t69299, t69302, t69305, t69307, t69310, t69958, t69959, t69961);
        let (t69996, t70009) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2462(t4657, t5872, t1022, t1058, t1060, t1063, t11034, t11059, t11060, t1610, t18086, t18089, t18094, t18129, t21481, t21614, t21626, t21637, t21657, t3186, t3200, t3201, t43553, t43554, t4649, t4669, t4673, t4678, t47853, t5928);
        let (t70012, t70014, t70068) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2463(t225, t69840, t1049, t21390, t1058, t1060, t11034, t11059, t11060, t11065, t14608, t1632, t17876, t18103, t18117, t18131, t21617, t21643, t21644, t21647, t21650, t3186, t3188, t3200, t43470, t43473, t4649, t4684, t5914, t69996);
    (t69742, t69860, t69871, t69923, t69935, t69942, t69966, t70009, t70012, t70014, t70068)
}
