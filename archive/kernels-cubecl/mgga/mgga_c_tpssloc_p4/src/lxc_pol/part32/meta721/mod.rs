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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta721<F: Float>(t29809: F, t85639: F, t1251: F, t5392: F, t1751: F, t8034: F, t29822: F, t17635: F, t17691: F, t2128: F, t24589: F, t24601: F, t27382: F, t27388: F, t27433: F, t27434: F, t27444: F, t27549: F, t27774: F, t27820: F, t4936: F, t7287: F, t8002: F, t85652: F, t94297: F, t94354: F, t94363: F, t94365: F, t94395: F, t94458: F, t29624: F, t491: F, t1760: F, t607: F, t27381: F, t8009: F, t1186: F, t17686: F, t24567: F, t24602: F, t27411: F, t27415: F, t27441: F, t27445: F, t27751: F, t29803: F, t4723: F, t4728: F, t4930: F, t5398: F, t7283: F, t8010: F, t85642: F, t85661: F, t94369: F, t94796: F, t95890: F, t29585: F, t6686: F, t27548: F, t8020: F, t29614: F, t1653: F, t19225: F, t2155: F, t24590: F, t27426: F, t27446: F, t27776: F, t29690: F, t5059: F, t65203: F, t66822: F, t7288: F, t7300: F, t85674: F, t94374: F, t94378: F, t94514: F, t24574: F, t29804: F, t18525: F, t19249: F, t2123: F, t29532: F, t29808: F, t29812: F, t3487: F, t6140: F, t64595: F, t7295: F, t7392: F, t85701: F, t86403: F, t94427: F, t94436: F, t94439: F, t94446: F, t94451: F, t94456: F, t8015: F, t94490: F, t29682: F, t29691: F, t24880: F, t27392: F, t27406: F, t27437: F, t27761: F, t29536: F, t4945: F, t6146: F, t6268: F, t94475: F, t94476: F, t94492: F, t94494: F, t94525: F, t29554: F, t1240: F, t6267: F, t2122: F, t29817: F, t1184: F, t6145: F, t1409: F, t1761: F, t19234: F, t27416: F, t27799: F, t27800: F, t5088: F, t7356: F, t85807: F, t86415: F, t94535: F, t95836: F, t3597: F, t6243: F, t6238: F, t7299: F, t1090: F, t18241: F, t19120: F, t19214: F, t19226: F, t2121: F, t225: F, t27403: F, t27438: F, t29678: F, t29798: F, t462: F, t497: F, t6244: F, t66845: F, t7285: F, t7286: F, t7296: F, t7302: F, t7351: F, t94628: F, t94631: F, t7284: F, t29546: F, t11605: F, t1238: F, t24893: F, t27742: F, t27784: F, t27792: F, t27821: F, t27826: F, t27830: F, t29794: F, t3598: F, t5060: F, t5089: F, t8087: F, t94648: F, t94656: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t103132, t103164) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2290::<F>(t29809, t85639, t1251, t5392, t1751, t8034, t29822, t17635, t17691, t2128, t24589, t24601, t27382, t27388, t27433, t27434, t27444, t27549, t27774, t27820, t4936, t7287, t8002, t85652, t94297, t94354, t94363, t94365, t94395, t94458);
        let t103213 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2291::<F>(t29624, t491, t1760, t607, t27381, t8009, t103132, t1186, t1251, t17686, t2128, t24567, t24589, t24601, t24602, t27411, t27415, t27441, t27445, t27549, t27751, t27820, t29803, t4723, t4728, t4930, t5398, t7283, t7287, t8010, t85642, t85661, t94369, t94395, t94458, t94796, t95890);
        let t103218 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2292::<F>(t29585, t6686);
        let (t103223, t103258) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2293::<F>(t27548, t8020, t29614, t491, t103218, t1653, t17691, t19225, t2155, t24589, t24590, t24601, t27388, t27426, t27433, t27445, t27446, t27549, t27774, t27776, t29690, t5059, t65203, t66822, t7283, t7287, t7288, t7300, t8002, t85674, t94374, t94378, t94395, t94514);
        let t103279 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2294::<F>(t24574, t29804, t18525, t19249, t2123, t2155, t24589, t24590, t29532, t29808, t29812, t3487, t6140, t64595, t7283, t7295, t7392, t85701, t86403, t94427, t94436, t94439, t94446, t94451, t94456);
        let t103303 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2295::<F>(t8015, t94490, t24574, t29682, t29691, t24589, t24880, t27392, t27406, t27437, t27761, t29536, t3487, t4945, t6146, t6268, t7283, t7295, t94475, t94476, t94492, t94494, t94514, t94525);
        let (t103314, t103337, t103341) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2296::<F>(t24574, t29554, t1240, t6267, t2122, t29817, t1184, t6145, t1186, t1409, t1761, t19234, t19249, t24589, t24601, t24602, t27406, t27416, t27437, t27799, t27800, t29690, t29808, t5088, t7283, t7356, t85807, t86415, t94458, t94535, t95836);
        let (t103345, t103377) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2297::<F>(t3597, t6243, t6238, t7299, t1090, t18241, t19120, t19214, t19226, t2121, t2155, t225, t24589, t24601, t24880, t27403, t27406, t27438, t29678, t29798, t3487, t462, t497, t6244, t66845, t7283, t7285, t7286, t7296, t7302, t7351, t94395, t94628, t94631);
        let t103415 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2298::<F>(t6238, t7284, t24574, t29546, t103314, t1090, t11605, t1238, t1251, t1761, t24589, t24601, t24893, t27382, t27406, t27742, t27784, t27792, t27821, t27826, t27830, t29794, t3598, t4930, t4945, t5059, t5060, t5089, t6244, t7283, t7287, t8087, t94395, t94648, t94656);
    (t103164, t103213, t103218, t103223, t103258, t103279, t103303, t103337, t103341, t103345, t103377, t103415)
}
