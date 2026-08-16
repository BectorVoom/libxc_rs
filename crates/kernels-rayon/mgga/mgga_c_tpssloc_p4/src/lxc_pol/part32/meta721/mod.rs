//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta721 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2290;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2291;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2292;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2293;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2294;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2295;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2296;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2297;
use chunk8::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2298;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta721(t29809: f64, t85639: f64, t1251: f64, t5392: f64, t1751: f64, t8034: f64, t29822: f64, t17635: f64, t17691: f64, t2128: f64, t24589: f64, t24601: f64, t27382: f64, t27388: f64, t27433: f64, t27434: f64, t27444: f64, t27549: f64, t27774: f64, t27820: f64, t4936: f64, t7287: f64, t8002: f64, t85652: f64, t94297: f64, t94354: f64, t94363: f64, t94365: f64, t94395: f64, t94458: f64, t29624: f64, t491: f64, t1760: f64, t607: f64, t27381: f64, t8009: f64, t1186: f64, t17686: f64, t24567: f64, t24602: f64, t27411: f64, t27415: f64, t27441: f64, t27445: f64, t27751: f64, t29803: f64, t4723: f64, t4728: f64, t4930: f64, t5398: f64, t7283: f64, t8010: f64, t85642: f64, t85661: f64, t94369: f64, t94796: f64, t95890: f64, t29585: f64, t6686: f64, t27548: f64, t8020: f64, t29614: f64, t1653: f64, t19225: f64, t2155: f64, t24590: f64, t27426: f64, t27446: f64, t27776: f64, t29690: f64, t5059: f64, t65203: f64, t66822: f64, t7288: f64, t7300: f64, t85674: f64, t94374: f64, t94378: f64, t94514: f64, t24574: f64, t29804: f64, t18525: f64, t19249: f64, t2123: f64, t29532: f64, t29808: f64, t29812: f64, t3487: f64, t6140: f64, t64595: f64, t7295: f64, t7392: f64, t85701: f64, t86403: f64, t94427: f64, t94436: f64, t94439: f64, t94446: f64, t94451: f64, t94456: f64, t8015: f64, t94490: f64, t29682: f64, t29691: f64, t24880: f64, t27392: f64, t27406: f64, t27437: f64, t27761: f64, t29536: f64, t4945: f64, t6146: f64, t6268: f64, t94475: f64, t94476: f64, t94492: f64, t94494: f64, t94525: f64, t29554: f64, t1240: f64, t6267: f64, t2122: f64, t29817: f64, t1184: f64, t6145: f64, t1409: f64, t1761: f64, t19234: f64, t27416: f64, t27799: f64, t27800: f64, t5088: f64, t7356: f64, t85807: f64, t86415: f64, t94535: f64, t95836: f64, t3597: f64, t6243: f64, t6238: f64, t7299: f64, t1090: f64, t18241: f64, t19120: f64, t19214: f64, t19226: f64, t2121: f64, t225: f64, t27403: f64, t27438: f64, t29678: f64, t29798: f64, t462: f64, t497: f64, t6244: f64, t66845: f64, t7285: f64, t7286: f64, t7296: f64, t7302: f64, t7351: f64, t94628: f64, t94631: f64, t7284: f64, t29546: f64, t11605: f64, t1238: f64, t24893: f64, t27742: f64, t27784: f64, t27792: f64, t27821: f64, t27826: f64, t27830: f64, t29794: f64, t3598: f64, t5060: f64, t5089: f64, t8087: f64, t94648: f64, t94656: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t103132, t103164) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2290(t29809, t85639, t1251, t5392, t1751, t8034, t29822, t17635, t17691, t2128, t24589, t24601, t27382, t27388, t27433, t27434, t27444, t27549, t27774, t27820, t4936, t7287, t8002, t85652, t94297, t94354, t94363, t94365, t94395, t94458);
        let t103213 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2291(t29624, t491, t1760, t607, t27381, t8009, t103132, t1186, t1251, t17686, t2128, t24567, t24589, t24601, t24602, t27411, t27415, t27441, t27445, t27549, t27751, t27820, t29803, t4723, t4728, t4930, t5398, t7283, t7287, t8010, t85642, t85661, t94369, t94395, t94458, t94796, t95890);
        let t103218 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2292(t29585, t6686);
        let (t103223, t103258) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2293(t27548, t8020, t29614, t491, t103218, t1653, t17691, t19225, t2155, t24589, t24590, t24601, t27388, t27426, t27433, t27445, t27446, t27549, t27774, t27776, t29690, t5059, t65203, t66822, t7283, t7287, t7288, t7300, t8002, t85674, t94374, t94378, t94395, t94514);
        let t103279 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2294(t24574, t29804, t18525, t19249, t2123, t2155, t24589, t24590, t29532, t29808, t29812, t3487, t6140, t64595, t7283, t7295, t7392, t85701, t86403, t94427, t94436, t94439, t94446, t94451, t94456);
        let t103303 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2295(t8015, t94490, t24574, t29682, t29691, t24589, t24880, t27392, t27406, t27437, t27761, t29536, t3487, t4945, t6146, t6268, t7283, t7295, t94475, t94476, t94492, t94494, t94514, t94525);
        let (t103314, t103337, t103341) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2296(t24574, t29554, t1240, t6267, t2122, t29817, t1184, t6145, t1186, t1409, t1761, t19234, t19249, t24589, t24601, t24602, t27406, t27416, t27437, t27799, t27800, t29690, t29808, t5088, t7283, t7356, t85807, t86415, t94458, t94535, t95836);
        let (t103345, t103377) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2297(t3597, t6243, t6238, t7299, t1090, t18241, t19120, t19214, t19226, t2121, t2155, t225, t24589, t24601, t24880, t27403, t27406, t27438, t29678, t29798, t3487, t462, t497, t6244, t66845, t7283, t7285, t7286, t7296, t7302, t7351, t94395, t94628, t94631);
        let t103415 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2298(t6238, t7284, t24574, t29546, t103314, t1090, t11605, t1238, t1251, t1761, t24589, t24601, t24893, t27382, t27406, t27742, t27784, t27792, t27821, t27826, t27830, t29794, t3598, t4930, t4945, t5059, t5060, t5089, t6244, t7283, t7287, t8087, t94395, t94648, t94656);
    (t103164, t103213, t103218, t103223, t103258, t103279, t103303, t103337, t103341, t103345, t103377, t103415)
}
