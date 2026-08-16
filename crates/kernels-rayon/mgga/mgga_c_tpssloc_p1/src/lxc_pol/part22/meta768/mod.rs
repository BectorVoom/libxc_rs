//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta768 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2602;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2603;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2604;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2605;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2606;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2607;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2608;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2609;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta768(t19051: f64, t4993: f64, t11784: f64, t1227: f64, t21762: f64, t248: f64, t11721: f64, t6218: f64, t11668: f64, t11692: f64, t15503: f64, t15700: f64, t1735: f64, t18241: f64, t19058: f64, t3515: f64, t3577: f64, t3578: f64, t45114: f64, t45197: f64, t4582: f64, t4972: f64, t4980: f64, t52548: f64, t52732: f64, t52897: f64, t5392: f64, t65464: f64, t65819: f64, t65881: f64, t65963: f64, t66533: f64, t70321: f64, t1174: f64, t135: f64, t22128: f64, t22132: f64, t11665: f64, t1216: f64, t15438: f64, t15507: f64, t18590: f64, t18955: f64, t19062: f64, t19072: f64, t21758: f64, t22158: f64, t45128: f64, t4984: f64, t5005: f64, t5024: f64, t50992: f64, t52759: f64, t65914: f64, t65920: f64, t65966: f64, t70330: f64, t18356: f64, t18975: f64, t21749: f64, t3431: f64, t11738: f64, t15569: f64, t15740: f64, t18225: f64, t18300: f64, t18321: f64, t18387: f64, t18969: f64, t19068: f64, t4950: f64, t4954: f64, t4969: f64, t5012: f64, t65541: f64, t65815: f64, t65935: f64, t22011: f64, t18375: f64, t5019: f64, t18946: f64, t19033: f64, t19056: f64, t19083: f64, t22208: f64, t3490: f64, t3506: f64, t44836: f64, t4989: f64, t5030: f64, t65884: f64, t65952: f64, t65992: f64, t65994: f64, t65996: f64, t65998: f64, t72445: f64, t14725: f64, t17635: f64, t18329: f64, t4889: f64, t18324: f64, t22136: f64, t18346: f64, t18580: f64, t19019: f64, t3440: f64, t52873: f64, t52893: f64, t66001: f64, t66015: f64, t66024: f64, t66027: f64, t71138: f64, t18371: f64, t1222: f64, t22175: f64, t1090: f64, t11728: f64, t18383: f64, t22312: f64, t66052: f64, t66054: f64, t66057: f64, t66073: f64, t66076: f64, t66079: f64, t66084: f64, t66092: f64, t1734: f64, t1089: f64, t11678: f64, t1215: f64, t15659: f64, t15701: f64, t15702: f64, t18237: f64, t18368: f64, t18395: f64, t18397: f64, t18401: f64, t4729: f64, t5046: f64, t52879: f64, t52903: f64, t5398: f64, t5979: f64, t607: f64, t6219: f64, t65469: f64, t66120: f64, t70458: f64, t18231: f64, t3961: f64, t22169: f64, t11539: f64, t21745: f64, t11546: f64, t15654: f64, t18342: f64, t22284: f64, t45134: f64, t4733: f64, t4987: f64, t5033: f64, t52919: f64, t6230: f64, t70316: f64, t70339: f64, t71133: f64, t71197: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t72577, t72593) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2602(t19051, t4993, t11784, t1227, t21762, t248, t11721, t6218, t11668, t11692, t15503, t15700, t1735, t18241, t19058, t3515, t3577, t3578, t45114, t45197, t4582, t4972, t4980, t52548, t52732, t52897, t5392, t65464, t65819, t65881, t65963, t66533, t70321);
        let t72622 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2603(t1174, t135, t22128, t22132, t11665, t1216, t1227, t15438, t15507, t18590, t18955, t19062, t19072, t21758, t22158, t3577, t45128, t4582, t4984, t5005, t5024, t50992, t52759, t65914, t65920, t65966, t70330);
        let t72654 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2604(t15503, t18356, t18975, t5024, t1174, t21749, t3431, t11738, t15569, t15740, t1735, t18225, t18300, t18321, t18387, t18969, t19068, t3577, t3578, t4582, t4950, t4954, t4969, t4980, t5012, t65541, t65815, t65935);
        let t72683 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2605(t1174, t135, t22011, t18375, t5019, t1216, t18946, t19033, t19056, t19083, t22208, t3490, t3506, t44836, t4582, t4950, t4954, t4989, t5030, t65884, t65952, t65992, t65994, t65996, t65998, t72445);
        let (t72688, t72712) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2606(t14725, t17635, t18329, t4889, t18324, t1174, t135, t22136, t18346, t18580, t19019, t3440, t45128, t5024, t52873, t52893, t66001, t66015, t66024, t66027, t71138);
        let t72735 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2607(t15740, t18371, t1222, t22175, t1090, t11728, t15569, t18300, t18383, t18946, t22312, t3578, t45114, t4582, t66052, t66054, t66057, t66073, t66076, t66079, t66084, t66092);
        let (t72767, t72783) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2608(t1734, t6218, t1089, t11678, t11692, t1215, t1227, t15569, t15659, t15700, t15701, t15702, t1735, t18237, t18321, t18368, t18395, t18397, t18401, t3577, t3578, t4582, t4729, t4972, t5046, t52879, t52903, t5398, t5979, t607, t6219, t65464, t65469, t66120, t70458);
        let (t72788, t72823) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2609(t18231, t3961, t1222, t22169, t11539, t1174, t21745, t11546, t11692, t1227, t15654, t18321, t18342, t19083, t22284, t3440, t3578, t45134, t4582, t4733, t4987, t4989, t5005, t5033, t52893, t52919, t6230, t70316, t70330, t70339, t71133, t71197);
    (t72577, t72593, t72622, t72654, t72683, t72688, t72712, t72735, t72767, t72783, t72788, t72823)
}
