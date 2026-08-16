//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta798 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2772;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2773;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2774;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2775;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2776;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2777;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2778;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta798(t5572: f64, t9541: f64, t4233: f64, t776: f64, t5527: f64, t828: f64, t5611: f64, t5624: f64, t9601: f64, t1512: f64, t47092: f64, t119: f64, t13222: f64, t13228: f64, t210: f64, t2571: f64, t2643: f64, t2647: f64, t41009: f64, t41053: f64, t4178: f64, t46587: f64, t46595: f64, t46611: f64, t46616: f64, t46618: f64, t46644: f64, t46649: f64, t46658: f64, t47039: f64, t58090: f64, t13257: f64, t4166: f64, t4184: f64, t10007: f64, t13251: f64, t13262: f64, t13263: f64, t13312: f64, t13350: f64, t16891: f64, t16944: f64, t16949: f64, t2633: f64, t2645: f64, t41063: f64, t4180: f64, t46597: f64, t46661: f64, t46663: f64, t46668: f64, t46675: f64, t46677: f64, t46679: f64, t46686: f64, t47017: f64, t5591: f64, t5593: f64, t58495: f64, t829: f64, t16673: f64, t2642: f64, t41424: f64, t5587: f64, t13278: f64, t4236: f64, t13186: f64, t13196: f64, t13254: f64, t13306: f64, t13316: f64, t1510: f64, t16893: f64, t16896: f64, t16924: f64, t2649: f64, t4172: f64, t4182: f64, t46698: f64, t46717: f64, t46733: f64, t46742: f64, t46748: f64, t9632: f64, t9642: f64, t9646: f64, t5584: f64, t16946: f64, t2697: f64, t16951: f64, t5614: f64, t9671: f64, t13223: f64, t13353: f64, t16662: f64, t16853: f64, t16859: f64, t2379: f64, t2553: f64, t2618: f64, t2623: f64, t2630: f64, t2701: f64, t4234: f64, t46692: f64, t46870: f64, t46874: f64, t47220: f64, t5544: f64, t58281: f64, t58340: f64, t817: f64, t819: f64, t820: f64, t843: f64, t9607: f64, t9613: f64, t46667: f64, t16903: f64, t9638: f64, t41008: f64, t5568: f64, t13225: f64, t16872: f64, t2686: f64, t41084: f64, t41086: f64, t46876: f64, t46882: f64, t46884: f64, t46886: f64, t46911: f64, t46918: f64, t46920: f64, t46926: f64, t46928: f64, t47285: f64, t9674: f64, t2639: f64, t13360: f64, t4257: f64, t58181: f64, t816: f64, t13351: f64, t13365: f64, t16928: f64, t16935: f64, t46565: f64, t46693: f64, t46930: f64, t46936: f64, t46951: f64, t46953: f64, t46960: f64, t46962: f64, t46974: f64, t46980: f64, t46998: f64, t831: f64, t16969: f64, t13258: f64, t41385: f64, t2629: f64, t842: f64, t13173: f64, t13177: f64, t13231: f64, t16836: f64, t16985: f64, t20981: f64, t2635: f64, t2681: f64, t40971: f64, t41096: f64, t4167: f64, t47012: f64, t47027: f64, t47262: f64, t5628: f64, t58139: f64, t847: f64, t849: f64, t9990: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t58552, t58581) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2772(t5572, t9541, t4233, t776, t5527, t828, t5611, t5624, t9601, t1512, t47092, t119, t13222, t13228, t210, t2571, t2643, t2647, t41009, t41053, t4178, t46587, t46595, t46611, t46616, t46618, t46644, t46649, t46658, t47039, t58090);
        let t58628 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2773(t13257, t4166, t4184, t10007, t13222, t13251, t13262, t13263, t13312, t13350, t16891, t16944, t16949, t2633, t2643, t2645, t2647, t41063, t4178, t4180, t46597, t46661, t46663, t46668, t46675, t46677, t46679, t46686, t47017, t5591, t5593, t58495, t829);
        let t58672 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2774(t16673, t2642, t41424, t5587, t13278, t4236, t13186, t13196, t13222, t13251, t13254, t13306, t13316, t13350, t1510, t16891, t16893, t16896, t16924, t2633, t2643, t2649, t4172, t4178, t4180, t4182, t46698, t46717, t46733, t46742, t46748, t58495, t58552, t9632, t9642, t9646);
        let t58725 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2775(t5584, t828, t16946, t2697, t16951, t5614, t9671, t13222, t13223, t13251, t13353, t1512, t16662, t16853, t16859, t2379, t2553, t2618, t2623, t2630, t2643, t2647, t2701, t4234, t46692, t46870, t46874, t47220, t5544, t58281, t58340, t776, t817, t819, t820, t843, t9607, t9613);
        let t58754 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2776(t1512, t46667, t16903, t9638, t41008, t5568, t13225, t13251, t13262, t16872, t2686, t41084, t41086, t46692, t46876, t46882, t46884, t46886, t46911, t46918, t46920, t46926, t46928, t47017, t47285);
        let t58789 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2777(t5614, t9674, t16859, t2639, t13360, t4257, t58181, t816, t13222, t13228, t13254, t13351, t13365, t16928, t16935, t2643, t4178, t46565, t46693, t46930, t46936, t46951, t46953, t46960, t46962, t46974, t46980, t46998, t5591, t831);
        let t58837 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2778(t16969, t9638, t13258, t16928, t41385, t5587, t16673, t2629, t58181, t842, t13173, t13177, t13222, t13231, t13262, t16836, t16872, t16985, t20981, t2379, t2623, t2635, t2643, t2681, t40971, t41096, t4167, t4178, t4236, t47012, t47027, t47262, t47285, t5527, t5591, t5628, t58139, t820, t843, t847, t849, t9990);
    (t58581, t58628, t58672, t58725, t58754, t58789, t58837)
}
