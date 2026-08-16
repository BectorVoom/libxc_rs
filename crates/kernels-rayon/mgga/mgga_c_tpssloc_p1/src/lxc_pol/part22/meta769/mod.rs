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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta769(t1174: f64, t18577: f64, t3440: f64, t3447: f64, t4889: f64, t4980: f64, t52953: f64, t52974: f64, t52988: f64, t52992: f64, t52994: f64, t53187: f64, t65703: f64, t66153: f64, t66155: f64, t66165: f64, t68513: f64, t71181: f64, t71185: f64, t1213: f64, t22244: f64, t248: f64, t3570: f64, t1227: f64, t21758: f64, t45268: f64, t11692: f64, t11697: f64, t22283: f64, t1216: f64, t15498: f64, t15569: f64, t1653: f64, t18360: f64, t18584: f64, t18941: f64, t19056: f64, t22301: f64, t22309: f64, t22314: f64, t3515: f64, t3577: f64, t3578: f64, t44858: f64, t44896: f64, t44965: f64, t45119: f64, t4582: f64, t5012: f64, t52897: f64, t53000: f64, t6203: f64, t72767: f64, t11665: f64, t11719: f64, t11721: f64, t1196: f64, t1215: f64, t15740: f64, t18300: f64, t18346: f64, t18965: f64, t19068: f64, t22154: f64, t44725: f64, t44863: f64, t45002: f64, t4987: f64, t5005: f64, t5011: f64, t52766: f64, t53034: f64, t66241: f64, t66255: f64, t67060: f64, t70458: f64, t72445: f64, t974: f64, t11678: f64, t22279: f64, t15453: f64, t1735: f64, t18206: f64, t19077: f64, t22258: f64, t3490: f64, t45020: f64, t45128: f64, t4972: f64, t52836: f64, t53079: f64, t53097: f64, t53099: f64, t66268: f64, t66273: f64, t66276: f64, t66324: f64, t70316: f64, t70339: f64, t22161: f64, t19025: f64, t5001: f64, t1090: f64, t11668: f64, t1218: f64, t15594: f64, t18215: f64, t18368: f64, t18590: f64, t18969: f64, t22299: f64, t44621: f64, t45044: f64, t5024: f64, t52628: f64, t53162: f64, t6211: f64, t66334: f64, t66337: f64, t71164: f64, t15438: f64, t15507: f64, t1748: f64, t19016: f64, t19062: f64, t19072: f64, t22288: f64, t22307: f64, t45112: f64, t45197: f64, t4984: f64, t6207: f64, t65706: f64, t65709: f64, t66360: f64, t66363: f64, t66398: f64, t70321: f64, t15737: f64, t18342: f64, t18594: f64, t19058: f64, t19101: f64, t53372: f64, t53399: f64, t6227: f64, t6232: f64, t66406: f64, t66408: f64, t66410: f64, t66413: f64, t66437: f64, t22243: f64, t486: f64, t1222: f64, t22116: f64, t15615: f64, t1743: f64, t18573: f64, t22197: f64, t3506: f64, t488: f64, t4978: f64, t51002: f64, t53271: f64, t53273: f64, t53274: f64, t66449: f64, t66452: f64, t66458: f64, t70330: f64, t18332: f64, t11734: f64, t1202: f64, t15503: f64, t18211: f64, t18383: f64, t18387: f64, t18948: f64, t21762: f64, t22174: f64, t22275: f64, t52615: f64, t6192: f64, t66500: f64, t66512: f64, t66515: f64, t66518: f64, t22153: f64, t13969: f64, t22274: f64, t22196: f64, t22015: f64, t18997: f64, t4733: f64, t52903: f64, t52995: f64, t53087: f64, t6219: f64, t66545: f64, t66554: f64, t66566: f64, t20246: f64, t972: f64, t1198: f64, t18364: f64, t45250: f64, t53249: f64, t53322: f64, t53434: f64, t53440: f64, t53453: f64, t53490: f64, t66571: f64, t66575: f64, t66597: f64, t66599: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t72842 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2610(t1174, t18577, t3440, t3447, t4889, t4980, t52953, t52974, t52988, t52992, t52994, t53187, t65703, t66153, t66155, t66165, t68513, t71181, t71185);
        let t72878 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2611(t1213, t22244, t248, t3570, t1227, t21758, t45268, t11692, t11697, t22283, t1216, t15498, t15569, t1653, t18360, t18584, t18941, t19056, t22301, t22309, t22314, t3515, t3577, t3578, t44858, t44896, t44965, t45119, t4582, t5012, t52897, t53000, t6203, t72767);
        let t72911 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2612(t11665, t11719, t11721, t1174, t1196, t1215, t1227, t15740, t18300, t18346, t18360, t18965, t19068, t22154, t44725, t44863, t45002, t4582, t4987, t5005, t5011, t52766, t53034, t66241, t66255, t67060, t70458, t72445, t974);
        let t72938 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2613(t11678, t11697, t22279, t1227, t15453, t1735, t18206, t19077, t22258, t3490, t3577, t45020, t45128, t4582, t4972, t52836, t53079, t53097, t53099, t66268, t66273, t66276, t66324, t70316, t70339);
        let t72970 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2614(t11697, t22161, t3577, t19025, t5001, t1090, t11668, t1174, t1218, t15569, t15594, t1735, t18215, t18368, t18590, t18969, t22299, t3578, t44621, t45044, t45119, t5024, t52628, t53162, t6211, t66334, t66337, t71164);
        let t72996 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2615(t1090, t11665, t1227, t15438, t15498, t15507, t15740, t1748, t19016, t19062, t19072, t22288, t22307, t3578, t45112, t45197, t4582, t4984, t4987, t6207, t65706, t65709, t66360, t66363, t66398, t70321);
        let t73019 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2616(t15594, t15737, t18342, t18594, t19058, t19101, t5005, t5024, t53372, t53399, t6207, t6227, t6232, t66406, t66408, t66410, t66413, t66437);
        let t73048 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2617(t22243, t486, t1222, t22116, t1216, t1227, t15615, t1743, t18573, t22197, t3490, t3506, t3515, t4582, t488, t4978, t51002, t53271, t53273, t53274, t66449, t66452, t66458, t70330, t70339);
        let t73078 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2618(t18332, t4889, t11668, t11734, t1202, t1216, t15503, t15740, t1735, t18211, t18383, t18387, t18948, t21762, t22174, t22275, t3577, t488, t52615, t6192, t66500, t66512, t66515, t66518);
        let t73108 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2619(t11697, t22153, t3577, t13969, t22274, t3515, t1227, t22196, t1222, t22015, t15740, t18584, t18965, t18997, t19077, t3447, t3578, t4733, t4889, t52903, t52995, t53087, t6219, t66545, t66554, t66566, t68513);
        let (t73113, t73126) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2620(t20246, t972, t1198, t15740, t18364, t3447, t45250, t53249, t53322, t53434, t53440, t53453, t53490, t6192, t66571, t66575, t66597, t66599, t68513);
    (t72842, t72878, t72911, t72938, t72970, t72996, t73019, t73048, t73078, t73108, t73113, t73126)
}
