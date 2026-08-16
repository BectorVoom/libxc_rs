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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta806(t212: f64, t5544: f64, t12998: f64, t686: f64, t776: f64, t13012: f64, t16798: f64, t16773: f64, t46843: f64, t16777: f64, t5527: f64, t46799: f64, t12984: f64, t4119: f64, t12971: f64, t13005: f64, t16771: f64, t16796: f64, t221: f64, t2379: f64, t2553: f64, t4127: f64, t4128: f64, t46770: f64, t46772: f64, t46780: f64, t46847: f64, t5555: f64, t9541: f64, t210: f64, t214: f64, t2571: f64, t41200: f64, t46782: f64, t46788: f64, t46790: f64, t46793: f64, t46796: f64, t46802: f64, t46806: f64, t46819: f64, t46828: f64, t46830: f64, t46836: f64, t58090: f64, t4255: f64, t41008: f64, t5550: f64, t16783: f64, t41196: f64, t118: f64, t16662: f64, t2576: f64, t794: f64, t16787: f64, t2563: f64, t16791: f64, t9546: f64, t2586: f64, t41146: f64, t9523: f64, t41209: f64, t41212: f64, t41217: f64, t46838: f64, t46844: f64, t46855: f64, t58139: f64, t787: f64, t59134: f64, t225: f64, t13222: f64, t13223: f64, t13228: f64, t16912: f64, t16969: f64, t237: f64, t249: f64, t2643: f64, t41130: f64, t41134: f64, t41139: f64, t41161: f64, t41341: f64, t41363: f64, t41365: f64, t41373: f64, t41386: f64, t4178: f64, t46692: f64, t47017: f64, t47093: f64, t47230: f64, t47267: f64, t5567: f64, t5571: f64, t59100: f64, t9559: f64, t9642: f64, t5624: f64, t9993: f64, t5628: f64, t16985: f64, t2697: f64, t1516: f64, t47275: f64, t47278: f64, t9601: f64, t2700: f64, t57043: f64, t247: f64, t4181: f64, t5619: f64, t9671: f64, t13229: f64, t13352: f64, t16976: f64, t20981: f64, t2701: f64, t2703: f64, t4281: f64, t4291: f64, t47269: f64, t47271: f64, t47273: f64, t47276: f64, t47279: f64, t47283: f64, t5585: f64, t820: f64, t843: f64, t16853: f64, t16673: f64, t2638: f64, t831: f64, t2693: f64, t5576: f64, t16965: f64, t9573: f64, t16997: f64, t838: f64, t16961: f64, t16888: f64, t9638: f64, t13191: f64, t13198: f64, t13350: f64, t1495: f64, t1510: f64, t17003: f64, t41410: f64, t4172: f64, t47333: f64, t47353: f64, t5587: f64, t58392: f64, t58439: f64, t58486: f64, t58540: f64, t58581: f64, t58628: f64, t58672: f64, t58725: f64, t58754: f64, t58789: f64, t58837: f64, t58887: f64, t59088: f64, t5611: f64, t852: f64, t17022: f64, t814: f64, t13176: f64, t13390: f64, t13407: f64, t16754: f64, t16762: f64, t17027: f64, t17041: f64, t226: f64, t235: f64, t2617: f64, t2679: f64, t2728: f64, t2738: f64, t4166: f64, t4282: f64, t4286: f64, t4288: f64, t58340: f64, t58345: f64, t812: f64, t829: f64) -> (f64, f64, f64, f64, f64) {
        let (t59135, t59138, t59140, t59154, t59156, t59162, t59165) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2798(t212, t5544, t12998, t686, t776, t13012, t16798, t16773, t46843, t16777, t5527, t46799);
        let t59178 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2799(t12984, t12998, t4119, t686, t12971, t13005, t16771, t16796, t221, t2379, t2553, t4127, t4128, t46770, t46772, t46780, t46847, t59138, t59140, t59154, t59156, t59165);
        let t59197 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2800(t5555, t9541, t210, t214, t2571, t41200, t46782, t46788, t46790, t46793, t46796, t46802, t46806, t46819, t46828, t46830, t46836, t58090);
        let (t59198, t59204, t59206, t59214, t59216) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2801(t4119, t4255, t41008, t5550, t16783, t41196, t118, t16662, t2576, t794, t16787, t2563);
        let t59227 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2802(t16791, t9546, t2586, t41146, t59162, t59135, t9523, t13005, t210, t214, t41209, t41212, t41217, t46838, t46844, t46855, t58139, t59198, t59204, t59206, t59214, t59216, t787);
        let (t59229, t59230, t59235) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2803(t59134, t59178, t59197, t59227, t225, t13222, t13223, t13228, t16912, t16969, t210, t237, t2379, t249, t2643, t41130, t41134, t41139, t41161, t41341, t41363, t41365, t41373, t41386, t4178, t46692, t47017, t47093, t47230, t47267, t5567, t5571, t59100, t9559, t9642);
        let (t59251, t59255, t59257, t59259, t59261, t59263, t59265, t59267) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2804(t5624, t9993, t5628, t16985, t2697, t1516, t47275, t47278, t9601, t2700, t57043, t247, t4181);
        let t59278 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2805(t5619, t9671, t13222, t13229, t13352, t16976, t20981, t2701, t2703, t4178, t4281, t4291, t47269, t47271, t47273, t47276, t47279, t47283, t5585, t58090, t59251, t59255, t59257, t59259, t59261, t59263, t59265, t59267, t820, t843);
        let (t59279, t59282, t59288, t59298, t59308, t59310) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2806(t16853, t2697, t16673, t2638, t831, t2693, t5576, t16965, t9573, t16997, t838, t16961);
        let t59324 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2807(t16888, t9638, t12971, t13005, t13191, t13198, t13350, t1495, t1510, t17003, t210, t221, t2553, t2571, t2643, t41410, t4172, t47333, t47353, t5567, t5571, t5587, t59198, t59279, t59282, t59288, t59298, t59308, t59310, t776, t9559, t9642);
        let t59328 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2808(t58392, t58439, t58486, t58540, t58581, t58628, t58672, t58725, t58754, t58789, t58837, t58887, t59088, t59235, t59278, t59324);
        let (t59331, t59351) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2809(t5611, t852, t17022, t814, t13176, t13390, t13407, t16673, t16754, t16762, t17027, t17041, t226, t235, t2617, t2679, t2728, t2738, t4166, t4281, t4282, t4286, t4288, t4291, t58340, t58345, t59328, t812, t829);
    (t59229, t59230, t59328, t59331, t59351)
}
