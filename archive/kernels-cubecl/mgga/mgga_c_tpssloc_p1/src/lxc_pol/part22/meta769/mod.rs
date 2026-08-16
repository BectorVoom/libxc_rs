//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta769 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2610;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2611;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2612;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2613;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2614;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2615;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2616;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2617;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2618;
use chunk9::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2619;
use chunk10::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2620;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta769<F: Float>(t1174: F, t18577: F, t3440: F, t3447: F, t4889: F, t4980: F, t52953: F, t52974: F, t52988: F, t52992: F, t52994: F, t53187: F, t65703: F, t66153: F, t66155: F, t66165: F, t68513: F, t71181: F, t71185: F, t1213: F, t22244: F, t248: F, t3570: F, t1227: F, t21758: F, t45268: F, t11692: F, t11697: F, t22283: F, t1216: F, t15498: F, t15569: F, t1653: F, t18360: F, t18584: F, t18941: F, t19056: F, t22301: F, t22309: F, t22314: F, t3515: F, t3577: F, t3578: F, t44858: F, t44896: F, t44965: F, t45119: F, t4582: F, t5012: F, t52897: F, t53000: F, t6203: F, t72767: F, t11665: F, t11719: F, t11721: F, t1196: F, t1215: F, t15740: F, t18300: F, t18346: F, t18965: F, t19068: F, t22154: F, t44725: F, t44863: F, t45002: F, t4987: F, t5005: F, t5011: F, t52766: F, t53034: F, t66241: F, t66255: F, t67060: F, t70458: F, t72445: F, t974: F, t11678: F, t22279: F, t15453: F, t1735: F, t18206: F, t19077: F, t22258: F, t3490: F, t45020: F, t45128: F, t4972: F, t52836: F, t53079: F, t53097: F, t53099: F, t66268: F, t66273: F, t66276: F, t66324: F, t70316: F, t70339: F, t22161: F, t19025: F, t5001: F, t1090: F, t11668: F, t1218: F, t15594: F, t18215: F, t18368: F, t18590: F, t18969: F, t22299: F, t44621: F, t45044: F, t5024: F, t52628: F, t53162: F, t6211: F, t66334: F, t66337: F, t71164: F, t15438: F, t15507: F, t1748: F, t19016: F, t19062: F, t19072: F, t22288: F, t22307: F, t45112: F, t45197: F, t4984: F, t6207: F, t65706: F, t65709: F, t66360: F, t66363: F, t66398: F, t70321: F, t15737: F, t18342: F, t18594: F, t19058: F, t19101: F, t53372: F, t53399: F, t6227: F, t6232: F, t66406: F, t66408: F, t66410: F, t66413: F, t66437: F, t22243: F, t486: F, t1222: F, t22116: F, t15615: F, t1743: F, t18573: F, t22197: F, t3506: F, t488: F, t4978: F, t51002: F, t53271: F, t53273: F, t53274: F, t66449: F, t66452: F, t66458: F, t70330: F, t18332: F, t11734: F, t1202: F, t15503: F, t18211: F, t18383: F, t18387: F, t18948: F, t21762: F, t22174: F, t22275: F, t52615: F, t6192: F, t66500: F, t66512: F, t66515: F, t66518: F, t22153: F, t13969: F, t22274: F, t22196: F, t22015: F, t18997: F, t4733: F, t52903: F, t52995: F, t53087: F, t6219: F, t66545: F, t66554: F, t66566: F, t20246: F, t972: F, t1198: F, t18364: F, t45250: F, t53249: F, t53322: F, t53434: F, t53440: F, t53453: F, t53490: F, t66571: F, t66575: F, t66597: F, t66599: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t72842 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2610::<F>(t1174, t18577, t3440, t3447, t4889, t4980, t52953, t52974, t52988, t52992, t52994, t53187, t65703, t66153, t66155, t66165, t68513, t71181, t71185);
        let t72878 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2611::<F>(t1213, t22244, t248, t3570, t1227, t21758, t45268, t11692, t11697, t22283, t1216, t15498, t15569, t1653, t18360, t18584, t18941, t19056, t22301, t22309, t22314, t3515, t3577, t3578, t44858, t44896, t44965, t45119, t4582, t5012, t52897, t53000, t6203, t72767);
        let t72911 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2612::<F>(t11665, t11719, t11721, t1174, t1196, t1215, t1227, t15740, t18300, t18346, t18360, t18965, t19068, t22154, t44725, t44863, t45002, t4582, t4987, t5005, t5011, t52766, t53034, t66241, t66255, t67060, t70458, t72445, t974);
        let t72938 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2613::<F>(t11678, t11697, t22279, t1227, t15453, t1735, t18206, t19077, t22258, t3490, t3577, t45020, t45128, t4582, t4972, t52836, t53079, t53097, t53099, t66268, t66273, t66276, t66324, t70316, t70339);
        let t72970 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2614::<F>(t11697, t22161, t3577, t19025, t5001, t1090, t11668, t1174, t1218, t15569, t15594, t1735, t18215, t18368, t18590, t18969, t22299, t3578, t44621, t45044, t45119, t5024, t52628, t53162, t6211, t66334, t66337, t71164);
        let t72996 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2615::<F>(t1090, t11665, t1227, t15438, t15498, t15507, t15740, t1748, t19016, t19062, t19072, t22288, t22307, t3578, t45112, t45197, t4582, t4984, t4987, t6207, t65706, t65709, t66360, t66363, t66398, t70321);
        let t73019 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2616::<F>(t15594, t15737, t18342, t18594, t19058, t19101, t5005, t5024, t53372, t53399, t6207, t6227, t6232, t66406, t66408, t66410, t66413, t66437);
        let t73048 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2617::<F>(t22243, t486, t1222, t22116, t1216, t1227, t15615, t1743, t18573, t22197, t3490, t3506, t3515, t4582, t488, t4978, t51002, t53271, t53273, t53274, t66449, t66452, t66458, t70330, t70339);
        let t73078 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2618::<F>(t18332, t4889, t11668, t11734, t1202, t1216, t15503, t15740, t1735, t18211, t18383, t18387, t18948, t21762, t22174, t22275, t3577, t488, t52615, t6192, t66500, t66512, t66515, t66518);
        let t73108 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2619::<F>(t11697, t22153, t3577, t13969, t22274, t3515, t1227, t22196, t1222, t22015, t15740, t18584, t18965, t18997, t19077, t3447, t3578, t4733, t4889, t52903, t52995, t53087, t6219, t66545, t66554, t66566, t68513);
        let (t73113, t73126) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2620::<F>(t20246, t972, t1198, t15740, t18364, t3447, t45250, t53249, t53322, t53434, t53440, t53453, t53490, t6192, t66571, t66575, t66597, t66599, t68513);
    (t72842, t72878, t72911, t72938, t72970, t72996, t73019, t73048, t73078, t73108, t73113, t73126)
}
