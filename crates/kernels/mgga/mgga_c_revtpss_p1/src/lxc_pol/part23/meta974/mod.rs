//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta974 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3310;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3311;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3312;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3313;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3314;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3315;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3316;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta974<F: Float>(t23087: F, t47672: F, t1448: F, t1907: F, t22483: F, t22496: F, t27153: F, t28198: F, t4139: F, t47088: F, t47092: F, t47096: F, t47098: F, t5541: F, t73407: F, t73499: F, t85976: F, t85979: F, t1868: F, t5778: F, t22809: F, t566: F, t1353: F, t1450: F, t198: F, t22813: F, t47113: F, t47116: F, t47118: F, t47122: F, t47124: F, t5536: F, t5542: F, t85987: F, t85989: F, t85990: F, t23059: F, t4147: F, t9593: F, t6836: F, t21969: F, t40076: F, t40079: F, t47152: F, t48327: F, t48330: F, t48332: F, t48334: F, t5532: F, t5591: F, t85993: F, t85994: F, t1312: F, t13426: F, t1518: F, t18227: F, t18245: F, t21881: F, t22633: F, t2322: F, t27123: F, t28219: F, t30138: F, t4248: F, t4292: F, t5523: F, t5920: F, t670: F, t75439: F, t75931: F, t75941: F, t7889: F, t85308: F, t85329: F, t85360: F, t1310: F, t1315: F, t1453: F, t18235: F, t1847: F, t21658: F, t21937: F, t22279: F, t22287: F, t22475: F, t22506: F, t22634: F, t22758: F, t23068: F, t23071: F, t23094: F, t25045: F, t39483: F, t39520: F, t39747: F, t39750: F, t39756: F, t39760: F, t39783: F, t39786: F, t39791: F, t39795: F, t39989: F, t40067: F, t40072: F, t4254: F, t46988: F, t46992: F, t47067: F, t47070: F, t47072: F, t47084: F, t47086: F, t47109: F, t47131: F, t47138: F, t47140: F, t47142: F, t48224: F, t48226: F, t48248: F, t48266: F, t48268: F, t48279: F, t48300: F, t48303: F, t48312: F, t49541: F, t511: F, t5528: F, t5627: F, t569: F, t5787: F, t651: F, t6773: F, t6934: F, t7732: F, t85890: F, t85891: F, t85893: F, t85900: F, t85903: F, t85904: F, t85908: F, t85909: F, t85910: F, t85920: F, t85921: F, t85922: F, t85972: F, t85973: F, t85974: F, t85975: F, t85980: F, t85981: F, t85982: F, t85984: F, t85986: F, t85991: F, t85992: F, t86728: F, t86731: F, t86741: F, t86751: F, t86753: F, t86764: F, t86771: F, t86782: F, t85312: F, t85343: F, t85373: F, t1921: F, t6936: F, t1913: F, t6951: F, t25072: F, t571: F, t116: F, t117: F, t1459: F, t1461: F, t18207: F, t1916: F, t1918: F, t22544: F, t22556: F, t22559: F, t22565: F, t22568: F, t25055: F, t25063: F, t25066: F, t25069: F, t572: F, t573: F, t5795: F, t5801: F, t5802: F, t5805: F, t5883: F, t60595: F, t6941: F, t6945: F, t6948: F, param_d: F, t1456: F, t1458: F, t1464: F, t1914: F, t22533: F, t22571: F, t25049: F, t3: F, t575: F, t5790: F, t5808: F, t60620: F, t60624: F, t60629: F, t6937: F, t75720: F, t75727: F, t75796: F, t75808: F) -> F {
        let t86804 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3310::<F>(t23087, t47672, t1448, t1907, t22483, t22496, t27153, t28198, t4139, t47088, t47092, t47096, t47098, t5541, t73407, t73499, t85976, t85979);
        let t86823 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3311::<F>(t1868, t5778, t22809, t566, t1353, t1448, t1450, t198, t22813, t4139, t47113, t47116, t47118, t47122, t47124, t5536, t5542, t85987, t85989, t85990);
        let (t86825, t86828, t86846) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3312::<F>(t23059, t4147, t23087, t9593, t566, t6836, t198, t21969, t40076, t40079, t4139, t47152, t48327, t48330, t48332, t48334, t5532, t5591, t85993, t85994);
        let t86889 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3313::<F>(t1312, t13426, t1518, t18227, t18245, t21881, t22633, t2322, t27123, t28219, t30138, t4248, t4292, t5523, t5920, t670, t75439, t75931, t75941, t7889, t85308, t85329, t85360);
        let t86891 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3314::<F>(t1310, t1315, t1353, t1448, t1453, t1518, t18235, t1847, t21658, t21937, t22279, t22287, t22475, t22496, t22506, t22633, t22634, t22758, t23068, t23071, t23094, t2322, t25045, t39483, t39520, t39747, t39750, t39756, t39760, t39783, t39786, t39791, t39795, t39989, t40067, t40072, t4139, t4254, t46988, t46992, t47067, t47070, t47072, t47084, t47086, t47109, t47131, t47138, t47140, t47142, t48224, t48226, t48248, t48266, t48268, t48279, t48300, t48303, t48312, t49541, t511, t5528, t5532, t5536, t5541, t5542, t5591, t5627, t569, t5778, t5787, t651, t6773, t6934, t7732, t85890, t85891, t85893, t85900, t85903, t85904, t85908, t85909, t85910, t85920, t85921, t85922, t85972, t85973, t85974, t85975, t85980, t85981, t85982, t85984, t85986, t85991, t85992, t86728, t86731, t86741, t86751, t86753, t86764, t86771, t86782, t86804, t86823, t86825, t86828, t86846, t86889);
        let (t86893, t86897, t86903, t86909, t86958) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3315::<F>(t85312, t85343, t85373, t86891, t1921, t6936, t1913, t6951, t25072, t571, t116, t117, t1459, t1461, t18207, t1916, t1918, t21881, t22544, t22556, t22559, t22565, t22568, t22633, t25055, t25063, t25066, t25069, t4292, t572, t573, t5795, t5801, t5802, t5805, t5883, t5920, t60595, t670, t6941, t6945, t6948, t75931, param_d);
        let tv4rho43 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3316::<F>(t1456, t1458, t1464, t1914, t1921, t22533, t22571, t25049, t25072, t3, t575, t5790, t5808, t60620, t60624, t60629, t6937, t6951, t75720, t75727, t75796, t75808, t86893, t86897, t86903, t86909, t86958);
    tv4rho43
}
