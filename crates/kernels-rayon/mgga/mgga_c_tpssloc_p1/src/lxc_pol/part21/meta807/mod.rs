//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta807 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2810;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2811;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2812;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2813;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2814;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2815;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2816;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2817;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2818;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2819;
use chunk10::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2820;
use chunk11::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2821;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta807(t2627: f64, t5631: f64, t13176: f64, t13417: f64, t13431: f64, t13434: f64, t1523: f64, t16823: f64, t17028: f64, t255: f64, t2617: f64, t2633: f64, t4162: f64, t4166: f64, t4296: f64, t4298: f64, t46528: f64, t5648: f64, t5653: f64, t59074: f64, t59230: f64, t812: f64, t860: f64, t9612: f64, t10076: f64, t13385: f64, t13390: f64, t13401: f64, t13404: f64, t13429: f64, t16753: f64, t16759: f64, t16811: f64, t16815: f64, t17027: f64, t17034: f64, t2684: f64, t2732: f64, t2740: f64, t4182: f64, t4281: f64, t4291: f64, t5575: f64, t5617: f64, t58226: f64, t58262: f64, t59331: f64, t829: f64, t4300: f64, t10049: f64, t10110: f64, t13053: f64, t13059: f64, t13461: f64, t1528: f64, t17050: f64, t17064: f64, t17070: f64, t17090: f64, t2597: f64, t2718: f64, t2719: f64, t2743: f64, t4147: f64, t4268: f64, t4273: f64, t4301: f64, t47609: f64, t47618: f64, t5657: f64, t5658: f64, t58143: f64, t58194: f64, t58224: f64, t58261: f64, t58304: f64, t58337: f64, t59351: f64, t855: f64, t858: f64, t866: f64, t9590: f64, t17100: f64, t225: f64, t13029: f64, t13042: f64, t13065: f64, t13072: f64, t1519: f64, t16804: f64, t17057: f64, t17092: f64, t259: f64, t2591: f64, t2713: f64, t2720: f64, t2742: f64, t4142: f64, t4265: f64, t47568: f64, t5636: f64, t5637: f64, t852: f64, t9593: f64, t17087: f64, t17060: f64, t13050: f64, t13071: f64, t13377: f64, t13460: f64, t13463: f64, t1492: f64, t1527: f64, t17022: f64, t17049: f64, t25168: f64, t46452: f64, t46488: f64, t47585: f64, t798: f64, t865: f64, t17095: f64, t17098: f64, t17052: f64, t218: f64, t252: f64, t2710: f64, t40890: f64, t46508: f64, t5558: f64, t59229: f64, t59328: f64, t10143: f64, t5660: f64, t12895: f64, t1877: f64, t193: f64, t202: f64, t2522: f64, t2749: f64, t39585: f64, t39590: f64, t39593: f64, t4119: f64, t58139: f64, t58973: f64, t58974: f64, t58975: f64, t58978: f64, t58979: f64, t58980: f64, t766: f64, t870: f64, t1484: f64, t2745: f64, t17109: f64, t2752: f64, t13471: f64, t16662: f64, t17116: f64, t262: f64, t41254: f64, t41258: f64, t41262: f64, t4307: f64, t4314: f64, t58983: f64, t58985: f64, t58986: f64, t58987: f64, t776: f64, t868: f64, t39658: f64, t58988: f64, t58989: f64, t58990: f64, t58991: f64, t58993: f64, t58996: f64, t58999: f64, t59001: f64, t59005: f64, t59008: f64, t59009: f64, t59011: f64, t16625: f64, t2379: f64, t47645: f64, t5502: f64, t5544: f64, t59014: f64, t59015: f64, t59016: f64, t59018: f64, t59019: f64, t59020: f64, t59023: f64, t59025: f64, t59027: f64, t9470: f64, t4303: f64, t2553: f64, t5527: f64, t59029: f64, t59031: f64, t59033: f64, t59034: f64, t59035: f64, t59038: f64, t59040: f64, t59043: f64, t59046: f64, t59049: f64, t57890: f64, t57901: f64, t57931: f64, t57955: f64, t57976: f64, t57994: f64, t58024: f64, t58036: f64, t58054: f64, t58063: f64, t58095: f64) -> f64 {
        let t59379 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2810(t2627, t5631, t13176, t13417, t13431, t13434, t1523, t16823, t17028, t255, t2617, t2633, t4162, t4166, t4296, t4298, t46528, t5648, t5653, t59074, t59230, t812, t860, t9612);
        let t59412 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2811(t10076, t13385, t13390, t13401, t13404, t13429, t16753, t16759, t16811, t16815, t17027, t17034, t2617, t2633, t2684, t2732, t2740, t4166, t4182, t4281, t4291, t5575, t5617, t58226, t58262, t59331, t812, t829);
        let t59434 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2812(t4300, t10049, t10110, t13053, t13059, t13461, t1528, t17050, t17064, t17070, t17090, t2597, t2718, t2719, t2743, t4147, t4268, t4273, t4301, t47609, t47618, t5657, t5658, t58143, t58194, t58224, t58261, t58304, t58337, t59351, t59379, t59412, t855, t858, t866, t9590);
        let t59475 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2813(t17100, t225, t10110, t13029, t13042, t13065, t13072, t13461, t1519, t1528, t16804, t17057, t17070, t17092, t259, t2591, t2713, t2720, t2742, t4142, t4147, t4265, t4273, t4301, t47568, t5631, t5636, t5637, t5658, t852, t855, t866, t9590, t9593);
        let t59518 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2814(t17087, t225, t17060, t13050, t13071, t13072, t13377, t13460, t13463, t1492, t1527, t1528, t17022, t17049, t17050, t17057, t25168, t259, t2597, t2713, t2718, t4147, t4268, t4273, t46452, t46488, t47585, t5637, t798, t855, t865, t866, t9593);
        let t59558 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2815(t17095, t225, t17098, t10049, t13042, t13059, t13463, t1528, t17052, t17064, t17090, t17092, t218, t252, t259, t2710, t2713, t2718, t2719, t2720, t2742, t2743, t40890, t4268, t4273, t4301, t46508, t5558, t5636, t5637, t5657, t59229, t59328, t855, t866);
        let t59571 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2816(t10143, t5660, t12895, t1877, t193, t202, t2522, t2749, t39585, t39590, t39593, t4119, t58139, t58973, t58974, t58975, t58978, t58979, t58980, t59434, t59475, t59518, t59558, t766, t870);
        let t59591 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2817(t1484, t2745, t17109, t2752, t13471, t16662, t17116, t1877, t2522, t262, t41254, t41258, t41262, t4307, t4314, t58983, t58985, t58986, t58987, t776, t868);
        let t59592 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2818(t39658, t58988, t58989, t58990, t58991, t58993, t58996, t58999, t59001, t59005, t59008, t59009, t59011);
        let t59602 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2819(t16625, t2379, t2522, t4314, t47645, t5502, t5544, t59014, t59015, t59016, t59018, t59019, t59020, t59023, t59025, t59027, t9470);
        let t59614 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2820(t4303, t16625, t193, t202, t2522, t2553, t2752, t4314, t5527, t59029, t59031, t59033, t59034, t59035, t59038, t59040, t59043, t59046, t59049, t9470);
        let t59618 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2821(t57890, t57901, t57931, t57955, t57976, t57994, t58024, t58036, t58054, t58063, t58095, t59571, t59591, t59592, t59602, t59614);
    t59618
}
