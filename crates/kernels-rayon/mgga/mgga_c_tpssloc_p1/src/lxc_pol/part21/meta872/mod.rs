//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta872 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3212;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3213;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3214;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3215;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3216;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3217;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3218;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3219;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3220;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta872(t63731: f64, t63733: f64, t63735: f64, t63737: f64, t63739: f64, t63741: f64, t63743: f64, t63745: f64, t63747: f64, t63752: f64, t63754: f64, t63757: f64, t63759: f64, t63763: f64, t19270: f64, t3633: f64, t4700: f64, t63765: f64, t63767: f64, t63769: f64, t63771: f64, t63829: f64, t64100: f64, t64253: f64, t64259: f64, t64433: f64, t65290: f64, t65293: f64, t65296: f64, t65299: f64, t5091: f64, t11947: f64, t6270: f64, t193: f64, t336: f64, t3637: f64, t3640: f64, t64436: f64, t64441: f64, t65301: f64, t65305: f64, t65307: f64, t65309: f64, t65312: f64, t65314: f64, t65319: f64, t65321: f64, t65324: f64, t65326: f64, t28: f64, t265: f64, t504: f64, t59618: f64, t64473: f64, t64510: f64, t64534: f64, t64545: f64, t66885: f64, t1081: f64, t1260: f64, t12606: f64, t13493: f64, t1409: f64, t15844: f64, t1649: f64, t16558: f64, t17133: f64, t1768: f64, t18196: f64, t19276: f64, t2250: f64, t2756: f64, t3231: f64, t3644: f64, t3966: f64, t47676: f64, t506: f64, t5099: f64, t52: f64, t5398: f64, t55677: f64, t5669: f64, t59627: f64, t59629: f64, t59631: f64, t5966: f64, t607: f64, t6279: f64, t873: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t113: f64, t12545: f64, t1271: f64, t12816: f64, t1393: f64, t1458: f64, t15857: f64, t16503: f64, t1778: f64, t1849: f64, t19289: f64, t19537: f64, t20098: f64, t20136: f64, t2312: f64, t2314: f64, t3652: f64, t3660: f64, t3929: f64, t4028: f64, t4034: f64, t510: f64, t513: f64, t5450: f64, t55568: f64, t55927: f64, t56110: f64, t56124: f64, t56148: f64, t56161: f64, t56174: f64, t56192: f64, t56212: f64, t56294: f64, t56364: f64, t56370: f64, t56389: f64, t56408: f64, t57801: f64, t57810: f64, t57815: f64, t57822: f64, t6287: f64, t6295: f64, t63261: f64, t6468: f64, t650: f64, t652: f64, t55998: f64, t56034: f64, t56075: f64, t1395: f64, t671: f64, t112: f64, t20148: f64, t12524: f64, t12813: f64, t1401: f64, t16521: f64, t16524: f64, t16538: f64, t16541: f64, t19534: f64, t20162: f64, t20173: f64, t20176: f64, t2363: f64, t3938: f64, t3941: f64, t4072: f64, t5456: f64, t55571: f64, t577: f64, t1851: f64, t5381: f64, t580: f64, t20186: f64, t576: f64, t6483: f64, t1404: f64, t6470: f64, t1858: f64, t5363: f64, t16507: f64, t3: f64, t5364: f64, t55368: f64, t55374: f64, t55376: f64, t55378: f64, t1396: f64, t1398: f64, t16546: f64, t1852: f64, t20149: f64, t3932: f64, t3946: f64, t45584: f64, t45588: f64, t55417: f64, t6471: f64) -> f64 {
        let t66886 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3212(t63731, t63733, t63735, t63737, t63739, t63741, t63743, t63745, t63747, t63752, t63754, t63757, t63759, t63763);
        let t66891 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3213(t19270, t3633, t4700, t63765, t63767, t63769, t63771, t63829, t64100, t64253, t64259, t64433, t65290, t65293, t65296, t65299);
        let t66901 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3214(t5091, t11947, t6270, t193, t336, t3637, t3640, t4700, t64436, t64441, t65301, t65305, t65307, t65309, t65312, t65314, t65319, t65321, t65324, t65326);
        let t66921 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3215(t28, t265, t504, t59618, t64473, t64510, t64534, t64545, t66885, t66886, t66891, t66901, t1081, t1260, t12606, t13493, t1409, t15844, t1649, t16558, t17133, t1768, t18196, t19276, t2250, t2756, t3231, t3644, t3966, t47676, t506, t5099, t52, t5398, t55677, t5669, t59627, t59629, t59631, t5966, t607, t6279, t873, dens_threshold, rho1, zeta_threshold);
        let t66935 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3216(t113, t12545, t1271, t12816, t1393, t1458, t15857, t16503, t1778, t1849, t19289, t19537, t20098, t20136, t2312, t2314, t3652, t3660, t3929, t4028, t4034, t510, t513, t5450, t55568, t55927, t56110, t56124, t56148, t56161, t56174, t56192, t56212, t56294, t56364, t56370, t56389, t56408, t57801, t57810, t57815, t57822, t6287, t6295, t63261, t6468, t650, t652, t66921);
        let (t66937, t66961) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3217(t55998, t56034, t56075, t66935, t1395, t671, t112, t20148, t12524, t12813, t1401, t1458, t16521, t16524, t16538, t16541, t19534, t20162, t20173, t20176, t2363, t3938, t3941, t4072, t5456, t55568, t55571, t577);
        let (t66964, t66967, t66976, t66987, t66989, t66991) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3218(t1851, t5381, t20148, t580, t20186, t576, t1395, t6483, t1404, t6470, t1858, t5363);
        let t66993 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3219(t16507, t1858, t3, t5364, t5381, t55368, t55374, t55376, t55378, t580, t66937, t66976, t66987, t66989, t66991);
        let tv4rho42 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3220(t1396, t1398, t1404, t16546, t1852, t20149, t20186, t3932, t3946, t45584, t45588, t55417, t6471, t6483, t66961, t66964, t66967, t66993);
    tv4rho42
}
