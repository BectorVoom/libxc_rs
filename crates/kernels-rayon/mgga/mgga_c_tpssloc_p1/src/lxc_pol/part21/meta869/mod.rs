//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta869 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3182;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3183;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3184;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3185;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3186;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3187;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3188;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3189;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3190;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3191;
use chunk10::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3192;
use chunk11::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3193;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta869(t15495: f64, t4997: f64, t15492: f64, t5019: f64, t15591: f64, t5002: f64, t1174: f64, t18237: f64, t3431: f64, t6187: f64, t698: f64, t1227: f64, t13969: f64, t18341: f64, t18345: f64, t1177: f64, t18943: f64, t3536: f64, t3555: f64, t52872: f64, t52875: f64, t55723: f64, t63294: f64, t63298: f64, t63302: f64, t974: f64, t18589: f64, t15743: f64, t5005: f64, t6177: f64, t11709: f64, t15455: f64, t15459: f64, t15463: f64, t15525: f64, t15535: f64, t15569: f64, t15612: f64, t15631: f64, t15650: f64, t1653: f64, t18321: f64, t19058: f64, t3552: f64, t3557: f64, t3560: f64, t3577: f64, t3578: f64, t5024: f64, t52906: f64, t53083: f64, t53087: f64, t11692: f64, t11697: f64, t18964: f64, t18583: f64, t11678: f64, t18367: f64, t18593: f64, t15640: f64, t15737: f64, t11668: f64, t15478: f64, t15659: f64, t18395: f64, t18946: f64, t19000: f64, t45114: f64, t45128: f64, t4723: f64, t52893: f64, t52897: f64, t52908: f64, t52917: f64, t52926: f64, t52932: f64, t53176: f64, t65014: f64, t65452: f64, t15503: f64, t19025: f64, t3535: f64, t1202: f64, t19032: f64, t15498: f64, t4993: f64, t15486: f64, t1090: f64, t1218: f64, t1232: f64, t15654: f64, t15708: f64, t18205: f64, t18941: f64, t3243: f64, t3447: f64, t3494: f64, t4582: f64, t4729: f64, t4987: f64, t5012: f64, t52935: f64, t52942: f64, t53249: f64, t55716: f64, t5971: f64, t61798: f64, t61910: f64, t6225: f64, t15590: f64, t5018: f64, t15507: f64, t15548: f64, t15438: f64, t15531: f64, t15555: f64, t15622: f64, t15627: f64, t18307: f64, t18346: f64, t3490: f64, t44858: f64, t44953: f64, t4980: f64, t52810: f64, t52836: f64, t52952: f64, t52973: f64, t52975: f64, t52987: f64, t53336: f64, t11719: f64, t11728: f64, t11738: f64, t15545: f64, t15620: f64, t15625: f64, t15656: f64, t18303: f64, t19056: f64, t3248: f64, t3506: f64, t3509: f64, t3516: f64, t44896: f64, t44968: f64, t44972: f64, t44976: f64, t52991: f64, t52993: f64, t52999: f64, t53001: f64, t6219: f64, t19057: f64, t11546: f64, t11665: f64, t15434: f64, t18360: f64, t18584: f64, t44996: f64, t45002: f64, t4889: f64, t4984: f64, t52601: f64, t52813: f64, t53023: f64, t53026: f64, t53033: f64, t53238: f64, t61855: f64, t6192: f64, t6230: f64, t63415: f64, t15608: f64, t15689: f64, t135: f64, t18996: f64, t18969: f64, t3440: f64, t45197: f64, t52704: f64, t53064: f64, t53067: f64, t53079: f64, t53093: f64, t53096: f64, t53099: f64, t53102: f64, t63315: f64, t1089: f64, t5011: f64, t607: f64, t1215: f64, t14749: f64, t15661: f64, t15663: f64, t15700: f64, t15701: f64, t15704: f64, t1735: f64, t18401: f64, t18959: f64, t3966: f64, t45020: f64, t4733: f64, t4972: f64, t52628: f64, t52903: f64, t53114: f64, t53116: f64, t53118: f64, t55666: f64, t18363: f64, t45124: f64, t18359: f64, t15740: f64, t18368: f64, t3562: f64, t45044: f64, t45049: f64, t45162: f64, t53135: f64, t53142: f64, t53155: f64, t53158: f64, t53161: f64, t53185: f64, t53472: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t65992, t65994, t65996, t65998, t66001, t66015, t66024) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3182(t15495, t4997, t15492, t5019, t15591, t5002, t1174, t18237, t3431, t6187, t698, t1227, t13969, t18341);
        let t66029 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3183(t1227, t13969, t18345, t1174, t1177, t18943, t3536, t3555, t52872, t52875, t55723, t63294, t63298, t63302, t65992, t65994, t65996, t65998, t66001, t66015, t66024, t974);
        let t66067 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3184(t1227, t13969, t18589, t15743, t5005, t1174, t6177, t698, t11709, t15455, t15459, t15463, t15525, t15535, t15569, t15612, t15631, t15650, t1653, t18321, t19058, t3552, t3557, t3560, t3577, t3578, t5024, t52906, t53083, t53087, t55723, t974);
        let (t66073, t66076, t66079, t66084, t66092) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3185(t11692, t11697, t18964, t18583, t3577, t11678, t18367, t1227, t13969, t18593, t15640, t15737);
        let t66111 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3186(t11668, t11678, t11692, t15478, t15569, t15659, t18395, t18946, t19000, t3577, t3578, t45114, t45128, t4723, t52893, t52897, t52908, t52917, t52926, t52932, t53176, t65014, t65452, t66073, t66076, t66079, t66084, t66092);
        let t66157 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3187(t15503, t15640, t19025, t3535, t1202, t19032, t15498, t4993, t15486, t5024, t1090, t11668, t11678, t1218, t1227, t1232, t15654, t15708, t18205, t18941, t3243, t3447, t3494, t3577, t3578, t45128, t4582, t4729, t4987, t5012, t52935, t52942, t53249, t55716, t5971, t61798, t61910, t6225);
        let t66185 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3188(t15590, t5018, t15507, t15548, t1218, t15438, t15503, t15531, t15535, t15555, t15622, t15627, t18307, t18346, t3490, t44858, t44953, t4980, t52810, t52836, t52952, t52973, t52975, t52987, t53336);
        let t66219 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3189(t11719, t11728, t11738, t15545, t15620, t15625, t15656, t18303, t19056, t3248, t3506, t3509, t3516, t3577, t3578, t44896, t44968, t44972, t44976, t4582, t5024, t52991, t52993, t52999, t53001, t6219);
        let t66254 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3190(t13969, t19057, t3506, t11546, t11665, t11668, t11692, t1174, t1227, t15434, t15622, t15627, t15737, t18360, t18584, t3243, t44996, t45002, t4582, t4889, t4984, t52601, t52813, t53023, t53026, t53033, t53238, t61855, t6192, t6230, t63415);
        let t66282 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3191(t15438, t15548, t15569, t15608, t15689, t4889, t1174, t135, t18996, t11665, t15650, t18969, t3440, t45197, t5005, t52704, t52897, t53064, t53067, t53079, t53093, t53096, t53099, t53102, t53176, t63315);
        let (t66310, t66326) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3192(t1089, t5011, t607, t15743, t5024, t11665, t11678, t11692, t1215, t1227, t14749, t15659, t15661, t15663, t15700, t15701, t15704, t1735, t18401, t18959, t3490, t3577, t3578, t3966, t45020, t45114, t4582, t4733, t4972, t52628, t52903, t53114, t53116, t53118, t55666, t6225);
        let t66353 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3193(t18363, t3577, t45124, t11697, t18359, t15459, t15463, t15478, t15631, t15740, t18321, t18368, t3562, t45044, t45049, t45162, t53135, t53142, t53155, t53158, t53161, t53185, t53472);
    (t66029, t66067, t66111, t66157, t66185, t66219, t66254, t66282, t66310, t66326, t66353)
}
