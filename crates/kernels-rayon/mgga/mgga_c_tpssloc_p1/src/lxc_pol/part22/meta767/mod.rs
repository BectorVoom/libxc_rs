//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta767 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2592;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2593;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2594;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2595;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2596;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2597;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2598;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2599;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2600;
use chunk9::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2601;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta767(t1227: f64, t21776: f64, t248: f64, t3521: f64, t18392: f64, t5005: f64, t15737: f64, t18356: f64, t19040: f64, t5024: f64, t11738: f64, t22299: f64, t3570: f64, t11728: f64, t22312: f64, t1174: f64, t1177: f64, t15495: f64, t6221: f64, t65552: f64, t65554: f64, t65558: f64, t65567: f64, t71189: f64, t71201: f64, t19033: f64, t4993: f64, t19046: f64, t5018: f64, t5023: f64, t6169: f64, t11546: f64, t1218: f64, t1230: f64, t1232: f64, t15498: f64, t1737: f64, t1748: f64, t19026: f64, t19087: f64, t22214: f64, t22218: f64, t3490: f64, t4889: f64, t5014: f64, t5030: f64, t6211: f64, t66147: f64, t66150: f64, t71148: f64, t71158: f64, t18321: f64, t5040: f64, t19002: f64, t19005: f64, t19047: f64, t19051: f64, t52628: f64, t65581: f64, t65598: f64, t65600: f64, t65605: f64, t65607: f64, t65613: f64, t71168: f64, t71177: f64, t1009: f64, t22113: f64, t1011: f64, t1212: f64, t18375: f64, t5002: f64, t18943: f64, t19080: f64, t65617: f64, t65619: f64, t65628: f64, t65632: f64, t65637: f64, t65647: f64, t65649: f64, t65651: f64, t66159: f64, t1730: f64, t19032: f64, t1017: f64, t1207: f64, t1210: f64, t22173: f64, t372: f64, t471: f64, t479: f64, t15507: f64, t19095: f64, t65660: f64, t65662: f64, t65664: f64, t65668: f64, t65670: f64, t65672: f64, t65674: f64, t65676: f64, t65681: f64, t22298: f64, t486: f64, t11668: f64, t11678: f64, t11692: f64, t15659: f64, t1735: f64, t18232: f64, t19000: f64, t3577: f64, t3578: f64, t45037: f64, t45114: f64, t45197: f64, t4582: f64, t4724: f64, t4729: f64, t4974: f64, t4978: f64, t4984: f64, t6225: f64, t6230: f64, t65464: f64, t65474: f64, t65545: f64, t65689: f64, t65691: f64, t72146: f64, t13969: f64, t22270: f64, t3506: f64, t15591: f64, t18301: f64, t18594: f64, t18955: f64, t18959: f64, t22280: f64, t45030: f64, t45162: f64, t4733: f64, t5012: f64, t52600: f64, t52601: f64, t52610: f64, t5975: f64, t70330: f64, t22257: f64, t21769: f64, t22157: f64, t45124: f64, t11709: f64, t1216: f64, t18303: f64, t18307: f64, t22246: f64, t22271: f64, t3536: f64, t5019: f64, t52810: f64, t53238: f64, t53472: f64, t5971: f64, t6227: f64, t66533: f64, t11697: f64, t22287: f64, t15569: f64, t18371: f64, t1090: f64, t18364: f64, t18948: f64, t22244: f64, t4950: f64, t4954: f64, t4989: f64, t52680: f64, t53083: f64, t53336: f64, t65803: f64, t66622: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t72273, t72285, t72287, t72289, t72293) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2592(t1227, t21776, t248, t3521, t18392, t5005, t15737, t18356, t19040, t5024, t11738, t22299, t3570);
        let t72299 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2593(t11728, t22312, t248, t3570, t1174, t1177, t15495, t6221, t65552, t65554, t65558, t65567, t71189, t71201, t72273, t72285, t72287, t72289, t72293);
        let t72333 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2594(t19033, t4993, t19046, t5018, t5023, t6169, t11546, t1174, t1218, t1227, t1230, t1232, t15498, t1737, t1748, t19026, t19087, t22214, t22218, t248, t3490, t4889, t5014, t5030, t6211, t66147, t66150, t71148, t71158);
        let t72357 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2595(t18321, t5040, t1174, t1177, t1748, t19002, t19005, t19047, t19051, t4889, t5014, t5030, t52628, t65581, t65598, t65600, t65605, t65607, t65613, t71168, t71177);
        let (t72361, t72380) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2596(t1009, t22113, t1011, t1212, t18375, t5002, t1218, t1737, t18943, t19080, t5014, t65617, t65619, t65628, t65632, t65637, t65647, t65649, t65651, t66159);
        let t72405 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2597(t1730, t19032, t1017, t1207, t1210, t22173, t372, t471, t479, t15507, t19095, t1218, t1232, t65660, t65662, t65664, t65668, t65670, t65672, t65674, t65676, t65681);
        let (t72445, t72452) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2598(t22298, t486, t11668, t11678, t11692, t15659, t1735, t18232, t19000, t19033, t3577, t3578, t45037, t45114, t45197, t4582, t4724, t4729, t4974, t4978, t4984, t6225, t6230, t65464, t65474, t65545, t65689, t65691, t72146);
        let t72484 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2599(t13969, t22270, t3506, t11678, t1227, t15591, t18301, t18594, t18955, t18959, t19051, t22280, t3577, t3578, t45030, t45162, t4582, t4733, t4974, t5005, t5012, t5024, t52600, t52601, t52610, t5975, t6221, t6225, t70330, t72445);
        let t72522 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2600(t1227, t13969, t22257, t21769, t248, t3521, t22157, t3577, t45124, t11668, t11709, t1216, t15659, t18303, t18307, t18943, t18959, t21776, t22246, t22271, t3506, t3536, t3578, t4582, t5005, t5012, t5019, t52810, t53238, t53472, t5971, t6227, t66533);
        let t72552 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2601(t11697, t22287, t3577, t15569, t18371, t1090, t1216, t15737, t18303, t18307, t18364, t18948, t19051, t21769, t22244, t3578, t4950, t4954, t4989, t52680, t53083, t53336, t65803, t66622);
    (t72299, t72333, t72357, t72361, t72380, t72405, t72445, t72452, t72484, t72522, t72552)
}
