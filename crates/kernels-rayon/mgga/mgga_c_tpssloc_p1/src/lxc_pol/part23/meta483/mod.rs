//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta483 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1463;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1464;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1465;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1466;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1467;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1468;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1469;
use chunk7::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1470;
use chunk8::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1471;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta483(t300: f64, t78874: f64, t78914: f64, t78944: f64, t79002: f64, t78335: f64, t78338: f64, t78344: f64, t78355: f64, t78357: f64, t78359: f64, t78361: f64, t78364: f64, t78367: f64, t78370: f64, t78373: f64, t78791: f64, t78792: f64, t78794: f64, t6224: f64, t11721: f64, t1213: f64, t1214: f64, t15503: f64, t19083: f64, t22246: f64, t22271: f64, t22309: f64, t248: f64, t45030: f64, t475: f64, t488: f64, t5002: f64, t53336: f64, t6164: f64, t6169: f64, t6211: f64, t65628: f64, t65632: f64, t65647: f64, t65664: f64, t65689: f64, t72403: f64, t1227: f64, t1230: f64, t15569: f64, t1653: f64, t19026: f64, t19051: f64, t22214: f64, t22218: f64, t22288: f64, t22307: f64, t3578: f64, t44828: f64, t45197: f64, t5005: f64, t6207: f64, t6221: f64, t6227: f64, t65541: f64, t65703: f64, t72470: f64, t72495: f64, t72501: f64, t77961: f64, t77969: f64, t11668: f64, t11678: f64, t11692: f64, t15740: f64, t19080: f64, t22158: f64, t22312: f64, t45114: f64, t52680: f64, t5971: f64, t5975: f64, t6225: f64, t6230: f64, t65819: f64, t72512: f64, t72530: f64, t72542: f64, t72556: f64, t72560: f64, t1735: f64, t1737: f64, t1748: f64, t21762: f64, t21769: f64, t3577: f64, t467: f64, t5979: f64, t6219: f64, t65935: f64, t72304: f64, t72307: f64, t72597: f64, t72600: f64, t72632: f64, t72634: f64, t72648: f64, t78506: f64, t11719: f64, t11728: f64, t11738: f64, t15438: f64, t15659: f64, t15737: f64, t1743: f64, t19056: f64, t22115: f64, t22275: f64, t22314: f64, t3506: f64, t3515: f64, t3585: f64, t4582: f64, t53472: f64, t65474: f64, t66015: f64, t72669: f64, t72673: f64, t73028: f64, t77965: f64, t1174: f64, t22149: f64, t22154: f64, t22301: f64, t3440: f64, t3508: f64, t45037: f64, t4889: f64, t5024: f64, t52836: f64, t66057: f64, t72703: f64, t72705: f64, t72708: f64, t72727: f64, t72733: f64, t72798: f64, t77981: f64, t78031: f64, t22162: f64, t22185: f64, t22284: f64, t22299: f64, t45119: f64, t45192: f64, t52903: f64, t53079: f64, t53099: f64, t6192: f64, t6232: f64, t65545: f64, t65815: f64, t72815: f64, t72849: f64, t72857: f64, t72864: f64, t75836: f64, t974: f64, t19033: f64, t44836: f64, t52766: f64, t6203: f64, t65963: f64, t65966: f64, t72363: f64, t72936: f64, t72959: f64, t77973: f64, t77977: f64, t78757: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t79005, t79006) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1463(t300, t78874, t78914, t78944, t79002, t78335, t78338, t78344, t78355, t78357, t78359, t78361, t78364, t78367, t78370, t78373);
        let (t79008, t79018, t79024) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1464(t78791, t78792, t78794, t79006, t6224, t11721, t1213, t1214, t15503, t19083, t22246, t22271, t22309, t248, t45030, t475, t488, t5002, t53336, t6164, t6169, t6211, t65628, t65632, t65647, t65664, t65689, t72403);
        let t79056 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1465(t1227, t1230, t15569, t1653, t19026, t19051, t22214, t22218, t22288, t22307, t248, t3578, t44828, t45197, t5005, t6207, t6211, t6221, t6227, t65541, t65703, t72470, t72495, t72501, t77961, t77969);
        let t79087 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1466(t11668, t11678, t11692, t15569, t15740, t1653, t19080, t22158, t22312, t3578, t45114, t52680, t5971, t5975, t6221, t6225, t6230, t65819, t72512, t72530, t72542, t72556, t72560);
        let t79120 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1467(t11668, t11692, t1735, t1737, t1748, t21762, t21769, t3577, t3578, t467, t5971, t5979, t6219, t6230, t65935, t72304, t72307, t72597, t72600, t72632, t72634, t72648, t78506);
        let t79160 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1468(t11719, t11728, t11738, t1227, t15438, t15659, t15737, t1735, t1743, t19056, t22115, t22271, t22275, t22314, t248, t3506, t3515, t3585, t4582, t488, t53472, t6225, t6230, t65474, t66015, t72669, t72673, t73028, t77965);
        let t79188 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1469(t1174, t1214, t1227, t1230, t15740, t22149, t22154, t22218, t22301, t248, t3440, t3508, t45037, t4889, t5024, t52836, t66057, t72703, t72705, t72708, t72727, t72733, t72798, t77981, t78031, t79018);
        let t79214 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1470(t1174, t15740, t1653, t22162, t22185, t22284, t22299, t3578, t45119, t45192, t5005, t52903, t53079, t53099, t6192, t6232, t65545, t65815, t72815, t72849, t72857, t72864, t75836, t974);
        let t79251 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1471(t1214, t1227, t1230, t1737, t19033, t19051, t19083, t22214, t22284, t248, t3515, t3585, t44836, t475, t5024, t52766, t6203, t6207, t6227, t6232, t65963, t65966, t72363, t72936, t72959, t77973, t77977, t78757, t79018);
    (t79005, t79008, t79018, t79024, t79056, t79087, t79120, t79160, t79188, t79214, t79251)
}
