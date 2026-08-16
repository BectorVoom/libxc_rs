//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta702 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2273;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2274;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2275;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2276;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2277;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2278;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2279;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2280;
use chunk8::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2281;
use chunk9::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2282;
use chunk10::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2283;
use chunk11::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2284;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta702(t23384: f64, t28681: f64, t1054: f64, t5943: f64, t1921: f64, t5914: f64, t6688: f64, t225: f64, t28505: f64, t28496: f64, t1066: f64, t17582: f64, t18165: f64, t23346: f64, t25406: f64, t25732: f64, t25757: f64, t25758: f64, t25826: f64, t28697: f64, t28713: f64, t3026: f64, t4557: f64, t6687: f64, t6691: f64, t6704: f64, t6705: f64, t82436: f64, t986: f64, t28488: f64, t10164: f64, t14545: f64, t14555: f64, t1599: f64, t17575: f64, t17588: f64, t23365: f64, t23588: f64, t25801: f64, t25810: f64, t28485: f64, t28495: f64, t3169: f64, t387: f64, t4540: f64, t4664: f64, t5838: f64, t6776: f64, t7600: f64, t7624: f64, t7625: f64, t88731: f64, t88753: f64, t28557: f64, t381: f64, t3173: f64, t5919: f64, t28702: f64, t82431: f64, t1052: f64, t1409: f64, t1626: f64, t1634: f64, t17686: f64, t23327: f64, t23329: f64, t23330: f64, t23336: f64, t23369: f64, t254: f64, t25429: f64, t25731: f64, t25759: f64, t28475: f64, t28499: f64, t3174: f64, t3966: f64, t4693: f64, t5944: f64, t6680: f64, t88035: f64, t88758: f64, t28510: f64, t28565: f64, t1065: f64, t14552: f64, t1635: f64, t17635: f64, t25423: f64, t25784: f64, t28470: f64, t4542: f64, t5398: f64, t5920: f64, t6816: f64, t83281: f64, t88145: f64, t884: f64, t10165: f64, t17691: f64, t23581: f64, t25430: f64, t25743: f64, t25755: f64, t28515: f64, t4665: f64, t6815: f64, t7553: f64, t88022: f64, t88023: f64, t88812: f64, t88845: f64, t88868: f64, t88932: f64, t28516: f64, t25749: f64, t7560: f64, t28594: f64, t17583: f64, t18047: f64, t18061: f64, t1920: f64, t1956: f64, t25420: f64, t345: f64, t4660: f64, t5844: f64, t61621: f64, t6699: f64, t6771: f64, t88882: f64, t89620: f64, t28519: f64, t25453: f64, t25778: f64, t28593: f64, t28679: f64, t388: f64, t82411: f64, t83344: f64, t88889: f64, t88915: f64, t990: f64, t99099: f64, t17667: f64, t23537: f64, t1622: f64, t17925: f64, t17962: f64, t23529: f64, t5861: f64, t5875: f64, t5880: f64, t6755: f64, t82848: f64, t82851: f64, t82956: f64, t83043: f64, t83061: f64, t83215: f64, t88249: f64, t88584: f64, t25577: f64, t4630: f64, t25580: f64, t4571: f64, t17906: f64, t6765: f64, t17884: f64, t17655: f64, t23541: f64, t1618: f64, t17972: f64, t23433: f64, t4575: f64, t5869: f64, t5900: f64, t82875: f64, t88251: f64, t88513: f64, t88591: f64, t17632: f64, t17637: f64, t17643: f64, t17688: f64, t17718: f64, t17976: f64, t17980: f64, t4585: f64, t4590: f64, t82885: f64, t83065: f64, t88281: f64, t18029: f64, t6754: f64, t1025: f64, t17693: f64, t17697: f64, t17734: f64, t23544: f64, t4636: f64, t4652: f64, t82914: f64, t88277: f64, t88305: f64, t88307: f64, t88388: f64, t17673: f64, t17984: f64, t25589: f64, t4596: f64, t4600: f64, t7578: f64, t83054: f64, t83058: f64, t88320: f64, t88321: f64, t88324: f64, t88335: f64, t88336: f64, t88339: f64, t88594: f64, t88600: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t99209, t99238) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2273(t23384, t28681, t1054, t5943, t1921, t5914, t6688, t225, t28505, t28496, t1066, t17582, t18165, t23346, t25406, t25732, t25757, t25758, t25826, t28697, t28713, t3026, t4557, t6687, t6691, t6704, t6705, t82436, t986);
        let t99271 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2274(t225, t28488, t10164, t1066, t14545, t14555, t1599, t17575, t17588, t1921, t23365, t23588, t25757, t25801, t25810, t28485, t28495, t3169, t387, t4540, t4664, t5838, t6687, t6776, t7600, t7624, t7625, t88731, t88753);
        let (t99296, t99313) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2275(t28557, t381, t3173, t5919, t1921, t28702, t82431, t1052, t1409, t1626, t1634, t17686, t23327, t23329, t23330, t23336, t23369, t254, t25429, t25731, t25759, t28475, t28499, t28713, t3169, t3174, t3966, t4693, t5944, t6680, t6687, t6691, t88035, t88758, t986);
        let t99353 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2276(t23384, t28510, t28565, t381, t1065, t14552, t1635, t17588, t17635, t23327, t23329, t23330, t23346, t23369, t25423, t25784, t28470, t28697, t3169, t4542, t5398, t5920, t6687, t6691, t6816, t7600, t83281, t88145, t884, t99209, t99296);
        let t99390 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2277(t10165, t1052, t1599, t17575, t17635, t17686, t17691, t23327, t23329, t23336, t23581, t25429, t25430, t25743, t25755, t28515, t4557, t4665, t5919, t6687, t6815, t6816, t7553, t88022, t88023, t88812, t88845, t88868, t88932);
        let t99422 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2278(t23384, t28470, t28516, t25749, t7560, t225, t28594, t1066, t1635, t17583, t18047, t18061, t1920, t1956, t23346, t25420, t25757, t25758, t345, t387, t4660, t5844, t61621, t6687, t6699, t6771, t88882, t89620, t986);
        let t99450 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2279(t23384, t28519, t1052, t23329, t23346, t25429, t25453, t25778, t28510, t28593, t28679, t3026, t3174, t388, t4660, t4665, t4693, t5943, t6815, t7624, t82411, t83344, t88889, t88915, t990, t99099);
        let t99492 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2280(t17667, t23537, t1622, t17925, t17962, t23529, t5861, t5875, t5880, t6755, t82848, t82851, t82956, t83043, t83061, t83215, t88249, t88584);
        let t99514 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2281(t25577, t4630, t25580, t4571, t17906, t6765, t17884, t17655, t23541, t1618, t17972, t23433, t23529, t4575, t5869, t5900, t82875, t88251, t88513, t88591);
        let t99535 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2282(t17632, t17637, t17643, t17688, t17718, t17976, t17980, t23541, t25580, t4585, t4590, t6765, t82885, t83065, t88281);
        let t99556 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2283(t18029, t6754, t1025, t1618, t1622, t17693, t17697, t17734, t23537, t23544, t25577, t25580, t4636, t4652, t5900, t6765, t82914, t88277, t88305, t88307, t88388);
        let t99571 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2284(t17673, t17984, t25589, t4596, t4600, t7578, t83054, t83058, t88320, t88321, t88324, t88335, t88336, t88339, t88594, t88600);
    (t99238, t99271, t99313, t99353, t99390, t99422, t99450, t99492, t99514, t99535, t99556, t99571)
}
