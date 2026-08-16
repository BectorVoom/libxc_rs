//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta806 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2798;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2799;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2800;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2801;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2802;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2803;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2804;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2805;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2806;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2807;
use chunk10::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2808;
use chunk11::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2809;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta806<F: Float>(t212: F, t5544: F, t12998: F, t686: F, t776: F, t13012: F, t16798: F, t16773: F, t46843: F, t16777: F, t5527: F, t46799: F, t12984: F, t4119: F, t12971: F, t13005: F, t16771: F, t16796: F, t221: F, t2379: F, t2553: F, t4127: F, t4128: F, t46770: F, t46772: F, t46780: F, t46847: F, t5555: F, t9541: F, t210: F, t214: F, t2571: F, t41200: F, t46782: F, t46788: F, t46790: F, t46793: F, t46796: F, t46802: F, t46806: F, t46819: F, t46828: F, t46830: F, t46836: F, t58090: F, t4255: F, t41008: F, t5550: F, t16783: F, t41196: F, t118: F, t16662: F, t2576: F, t794: F, t16787: F, t2563: F, t16791: F, t9546: F, t2586: F, t41146: F, t9523: F, t41209: F, t41212: F, t41217: F, t46838: F, t46844: F, t46855: F, t58139: F, t787: F, t59134: F, t225: F, t13222: F, t13223: F, t13228: F, t16912: F, t16969: F, t237: F, t249: F, t2643: F, t41130: F, t41134: F, t41139: F, t41161: F, t41341: F, t41363: F, t41365: F, t41373: F, t41386: F, t4178: F, t46692: F, t47017: F, t47093: F, t47230: F, t47267: F, t5567: F, t5571: F, t59100: F, t9559: F, t9642: F, t5624: F, t9993: F, t5628: F, t16985: F, t2697: F, t1516: F, t47275: F, t47278: F, t9601: F, t2700: F, t57043: F, t247: F, t4181: F, t5619: F, t9671: F, t13229: F, t13352: F, t16976: F, t20981: F, t2701: F, t2703: F, t4281: F, t4291: F, t47269: F, t47271: F, t47273: F, t47276: F, t47279: F, t47283: F, t5585: F, t820: F, t843: F, t16853: F, t16673: F, t2638: F, t831: F, t2693: F, t5576: F, t16965: F, t9573: F, t16997: F, t838: F, t16961: F, t16888: F, t9638: F, t13191: F, t13198: F, t13350: F, t1495: F, t1510: F, t17003: F, t41410: F, t4172: F, t47333: F, t47353: F, t5587: F, t58392: F, t58439: F, t58486: F, t58540: F, t58581: F, t58628: F, t58672: F, t58725: F, t58754: F, t58789: F, t58837: F, t58887: F, t59088: F, t5611: F, t852: F, t17022: F, t814: F, t13176: F, t13390: F, t13407: F, t16754: F, t16762: F, t17027: F, t17041: F, t226: F, t235: F, t2617: F, t2679: F, t2728: F, t2738: F, t4166: F, t4282: F, t4286: F, t4288: F, t58340: F, t58345: F, t812: F, t829: F) -> (F, F, F, F, F) {
        let (t59135, t59138, t59140, t59154, t59156, t59162, t59165) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2798::<F>(t212, t5544, t12998, t686, t776, t13012, t16798, t16773, t46843, t16777, t5527, t46799);
        let t59178 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2799::<F>(t12984, t12998, t4119, t686, t12971, t13005, t16771, t16796, t221, t2379, t2553, t4127, t4128, t46770, t46772, t46780, t46847, t59138, t59140, t59154, t59156, t59165);
        let t59197 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2800::<F>(t5555, t9541, t210, t214, t2571, t41200, t46782, t46788, t46790, t46793, t46796, t46802, t46806, t46819, t46828, t46830, t46836, t58090);
        let (t59198, t59204, t59206, t59214, t59216) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2801::<F>(t4119, t4255, t41008, t5550, t16783, t41196, t118, t16662, t2576, t794, t16787, t2563);
        let t59227 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2802::<F>(t16791, t9546, t2586, t41146, t59162, t59135, t9523, t13005, t210, t214, t41209, t41212, t41217, t46838, t46844, t46855, t58139, t59198, t59204, t59206, t59214, t59216, t787);
        let (t59229, t59230, t59235) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2803::<F>(t59134, t59178, t59197, t59227, t225, t13222, t13223, t13228, t16912, t16969, t210, t237, t2379, t249, t2643, t41130, t41134, t41139, t41161, t41341, t41363, t41365, t41373, t41386, t4178, t46692, t47017, t47093, t47230, t47267, t5567, t5571, t59100, t9559, t9642);
        let (t59251, t59255, t59257, t59259, t59261, t59263, t59265, t59267) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2804::<F>(t5624, t9993, t5628, t16985, t2697, t1516, t47275, t47278, t9601, t2700, t57043, t247, t4181);
        let t59278 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2805::<F>(t5619, t9671, t13222, t13229, t13352, t16976, t20981, t2701, t2703, t4178, t4281, t4291, t47269, t47271, t47273, t47276, t47279, t47283, t5585, t58090, t59251, t59255, t59257, t59259, t59261, t59263, t59265, t59267, t820, t843);
        let (t59279, t59282, t59288, t59298, t59308, t59310) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2806::<F>(t16853, t2697, t16673, t2638, t831, t2693, t5576, t16965, t9573, t16997, t838, t16961);
        let t59324 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2807::<F>(t16888, t9638, t12971, t13005, t13191, t13198, t13350, t1495, t1510, t17003, t210, t221, t2553, t2571, t2643, t41410, t4172, t47333, t47353, t5567, t5571, t5587, t59198, t59279, t59282, t59288, t59298, t59308, t59310, t776, t9559, t9642);
        let t59328 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2808::<F>(t58392, t58439, t58486, t58540, t58581, t58628, t58672, t58725, t58754, t58789, t58837, t58887, t59088, t59235, t59278, t59324);
        let (t59331, t59351) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2809::<F>(t5611, t852, t17022, t814, t13176, t13390, t13407, t16673, t16754, t16762, t17027, t17041, t226, t235, t2617, t2679, t2728, t2738, t4166, t4281, t4282, t4286, t4288, t4291, t58340, t58345, t59328, t812, t829);
    (t59229, t59230, t59328, t59331, t59351)
}
