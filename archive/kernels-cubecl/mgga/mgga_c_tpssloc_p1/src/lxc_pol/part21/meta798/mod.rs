//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta798 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2772;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2773;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2774;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2775;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2776;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2777;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2778;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta798<F: Float>(t5572: F, t9541: F, t4233: F, t776: F, t5527: F, t828: F, t5611: F, t5624: F, t9601: F, t1512: F, t47092: F, t119: F, t13222: F, t13228: F, t210: F, t2571: F, t2643: F, t2647: F, t41009: F, t41053: F, t4178: F, t46587: F, t46595: F, t46611: F, t46616: F, t46618: F, t46644: F, t46649: F, t46658: F, t47039: F, t58090: F, t13257: F, t4166: F, t4184: F, t10007: F, t13251: F, t13262: F, t13263: F, t13312: F, t13350: F, t16891: F, t16944: F, t16949: F, t2633: F, t2645: F, t41063: F, t4180: F, t46597: F, t46661: F, t46663: F, t46668: F, t46675: F, t46677: F, t46679: F, t46686: F, t47017: F, t5591: F, t5593: F, t58495: F, t829: F, t16673: F, t2642: F, t41424: F, t5587: F, t13278: F, t4236: F, t13186: F, t13196: F, t13254: F, t13306: F, t13316: F, t1510: F, t16893: F, t16896: F, t16924: F, t2649: F, t4172: F, t4182: F, t46698: F, t46717: F, t46733: F, t46742: F, t46748: F, t9632: F, t9642: F, t9646: F, t5584: F, t16946: F, t2697: F, t16951: F, t5614: F, t9671: F, t13223: F, t13353: F, t16662: F, t16853: F, t16859: F, t2379: F, t2553: F, t2618: F, t2623: F, t2630: F, t2701: F, t4234: F, t46692: F, t46870: F, t46874: F, t47220: F, t5544: F, t58281: F, t58340: F, t817: F, t819: F, t820: F, t843: F, t9607: F, t9613: F, t46667: F, t16903: F, t9638: F, t41008: F, t5568: F, t13225: F, t16872: F, t2686: F, t41084: F, t41086: F, t46876: F, t46882: F, t46884: F, t46886: F, t46911: F, t46918: F, t46920: F, t46926: F, t46928: F, t47285: F, t9674: F, t2639: F, t13360: F, t4257: F, t58181: F, t816: F, t13351: F, t13365: F, t16928: F, t16935: F, t46565: F, t46693: F, t46930: F, t46936: F, t46951: F, t46953: F, t46960: F, t46962: F, t46974: F, t46980: F, t46998: F, t831: F, t16969: F, t13258: F, t41385: F, t2629: F, t842: F, t13173: F, t13177: F, t13231: F, t16836: F, t16985: F, t20981: F, t2635: F, t2681: F, t40971: F, t41096: F, t4167: F, t47012: F, t47027: F, t47262: F, t5628: F, t58139: F, t847: F, t849: F, t9990: F) -> (F, F, F, F, F, F, F) {
        let (t58552, t58581) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2772::<F>(t5572, t9541, t4233, t776, t5527, t828, t5611, t5624, t9601, t1512, t47092, t119, t13222, t13228, t210, t2571, t2643, t2647, t41009, t41053, t4178, t46587, t46595, t46611, t46616, t46618, t46644, t46649, t46658, t47039, t58090);
        let t58628 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2773::<F>(t13257, t4166, t4184, t10007, t13222, t13251, t13262, t13263, t13312, t13350, t16891, t16944, t16949, t2633, t2643, t2645, t2647, t41063, t4178, t4180, t46597, t46661, t46663, t46668, t46675, t46677, t46679, t46686, t47017, t5591, t5593, t58495, t829);
        let t58672 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2774::<F>(t16673, t2642, t41424, t5587, t13278, t4236, t13186, t13196, t13222, t13251, t13254, t13306, t13316, t13350, t1510, t16891, t16893, t16896, t16924, t2633, t2643, t2649, t4172, t4178, t4180, t4182, t46698, t46717, t46733, t46742, t46748, t58495, t58552, t9632, t9642, t9646);
        let t58725 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2775::<F>(t5584, t828, t16946, t2697, t16951, t5614, t9671, t13222, t13223, t13251, t13353, t1512, t16662, t16853, t16859, t2379, t2553, t2618, t2623, t2630, t2643, t2647, t2701, t4234, t46692, t46870, t46874, t47220, t5544, t58281, t58340, t776, t817, t819, t820, t843, t9607, t9613);
        let t58754 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2776::<F>(t1512, t46667, t16903, t9638, t41008, t5568, t13225, t13251, t13262, t16872, t2686, t41084, t41086, t46692, t46876, t46882, t46884, t46886, t46911, t46918, t46920, t46926, t46928, t47017, t47285);
        let t58789 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2777::<F>(t5614, t9674, t16859, t2639, t13360, t4257, t58181, t816, t13222, t13228, t13254, t13351, t13365, t16928, t16935, t2643, t4178, t46565, t46693, t46930, t46936, t46951, t46953, t46960, t46962, t46974, t46980, t46998, t5591, t831);
        let t58837 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2778::<F>(t16969, t9638, t13258, t16928, t41385, t5587, t16673, t2629, t58181, t842, t13173, t13177, t13222, t13231, t13262, t16836, t16872, t16985, t20981, t2379, t2623, t2635, t2643, t2681, t40971, t41096, t4167, t4178, t4236, t47012, t47027, t47262, t47285, t5527, t5591, t5628, t58139, t820, t843, t847, t849, t9990);
    (t58581, t58628, t58672, t58725, t58754, t58789, t58837)
}
