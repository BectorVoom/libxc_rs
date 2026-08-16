//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta368 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1384;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1385;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1386;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1387;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1388;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1389;
use chunk6::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1390;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta368(t13985: f64, t4593: f64, t4582: f64, t3132: f64, t3069: f64, t4669: f64, t10231: f64, t4338: f64, t973: f64, t13542: f64, t977: f64, t10388: f64, t10424: f64, t10480: f64, t10876: f64, t10898: f64, t10949: f64, t13959: f64, t13963: f64, t13966: f64, t13972: f64, t13977: f64, t13982: f64, t1618: f64, t3073: f64, t3109: f64, t3130: f64, t4596: f64, t4652: f64, t13546: f64, t13555: f64, t2979: f64, t13528: f64, t13532: f64, t10214: f64, t13537: f64, t13969: f64, t4595: f64, t1616: f64, t2780: f64, t3071: f64, t2771: f64, t10408: f64, t1539: f64, t3121: f64, t3048: f64, t4571: f64, t10390: f64, t10891: f64, t10904: f64, t10937: f64, t10957: f64, t1622: f64, t3070: f64, t3098: f64, t4575: f64, t4600: f64, t4644: f64, t4630: f64, t4650: f64, t884: f64, t10436: f64, t10441: f64, t10449: f64, t10455: f64, t10460: f64, t10490: f64, t10496: f64, t10504: f64, t10511: f64, t10517: f64, t10863: f64, t10866: f64, t10871: f64, t4636: f64, t3108: f64, t4640: f64, t1611: f64, t3047: f64, t3103: f64, t4641: f64, t1040: f64, t4616: f64, t1044: f64, t13611: f64, t248: f64, t1023: f64, t13975: f64, t3041: f64, t1031: f64, t1612: f64, t3082: f64, t1025: f64, t1041: f64, t1046: f64, t10873: f64, t10883: f64, t10952: f64, t10965: f64, t3039: f64, t3117: f64, t378: f64, t4585: f64, t4590: f64, t2776: f64, t4584: f64, t4589: f64, t12652: f64, t4583: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t14004 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1384(t13985, t4593, t4582, t3132, t3069, t4669, t10231, t4338, t973, t13542, t977, t10388, t10424, t10480, t10876, t10898, t10949, t13959, t13963, t13966, t13972, t13977, t13982, t1618, t3073, t3109, t3130, t4596, t4652);
        let (t14006, t14009, t14012, t14015, t14018, t14027, t14032) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1385(t13546, t977, t13555, t2979, t13528, t13532, t10214, t13537, t13969, t4595, t3130, t1616, t2780);
        let t14050 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1386(t14032, t3071, t1616, t2771, t10408, t1539, t3121, t3048, t4571, t10390, t10891, t10904, t10937, t10957, t14006, t14009, t14012, t14015, t14018, t14027, t1622, t3070, t3098, t4575, t4596, t4600, t4644, t973);
        let t14074 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1387(t3109, t4630, t4650, t884, t3071, t10436, t10441, t10449, t10455, t10460, t10490, t10496, t10504, t10511, t10517, t10863, t10866, t10871, t1618, t1622, t3048, t3070, t4636);
        let (t14077, t14080, t14084, t14085, t14093, t14098) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1388(t3108, t4640, t1611, t3047, t3103, t4641, t1040, t4616, t1044, t13611, t248, t1023, t13975);
        let t14120 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1389(t14098, t4582, t3121, t4593, t3041, t1031, t4616, t1612, t3082, t1025, t1041, t1046, t10873, t10883, t10952, t10965, t14077, t14080, t14084, t14085, t14093, t1622, t3039, t3048, t3117, t378, t4585, t4590, t4600, t4636);
        let (t14122, t14126, t14130, t14136, t14139, t14142) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1390(t1539, t3132, t3071, t3041, t1616, t2776, t13969, t4584, t1041, t4589, t12652, t4583);
    (t14004, t14050, t14074, t14120, t14122, t14126, t14130, t14136, t14139, t14142)
}
