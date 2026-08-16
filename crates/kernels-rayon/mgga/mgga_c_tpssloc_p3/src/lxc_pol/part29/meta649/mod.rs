//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta649 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2158;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2159;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2160;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2161;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2162;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2163;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2164;
use chunk7::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2165;
use chunk8::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2166;
use chunk9::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2167;
use chunk10::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2168;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta649(t1880: f64, t23196: f64, t25224: f64, t23030: f64, t25205: f64, t23164: f64, t7479: f64, t82133: f64, t6552: f64, t82124: f64, t23237: f64, t25341: f64, t23204: f64, t25216: f64, t6562: f64, t1519: f64, t212: f64, t23171: f64, t6554: f64, t23270: f64, t25038: f64, t258: f64, t4119: f64, t776: f64, t25039: f64, t2553: f64, t25040: f64, t82074: f64, t87712: f64, t82294: f64, t25193: f64, t81591: f64, t10049: f64, t13053: f64, t6632: f64, t6663: f64, t7538: f64, t82296: f64, t9593: f64, t10109: f64, t10110: f64, t13029: f64, t13042: f64, t13050: f64, t13058: f64, t13071: f64, t13460: f64, t13463: f64, t1528: f64, t1902: f64, t1911: f64, t1912: f64, t22979: f64, t23191: f64, t23215: f64, t23278: f64, t23281: f64, t25168: f64, t25169: f64, t25170: f64, t25188: f64, t25200: f64, t25233: f64, t25329: f64, t25330: f64, t25348: f64, t259: f64, t2597: f64, t2713: f64, t2718: f64, t2719: f64, t2720: f64, t2743: f64, t4142: f64, t4147: f64, t4268: f64, t4272: f64, t4273: f64, t4301: f64, t47568: f64, t47609: f64, t6624: f64, t6627: f64, t6662: f64, t7517: f64, t7537: f64, t82070: f64, t82071: f64, t82076: f64, t82079: f64, t82131: f64, t82135: f64, t82197: f64, t82287: f64, t855: f64, t865: f64, t866: f64, t86905: f64, t86909: f64, t86911: f64, t86916: f64, t86923: f64, t86952: f64, t86955: f64, t86961: f64, t86968: f64, t86972: f64, t87005: f64, t87010: f64, t87013: f64, t87741: f64, t87792: f64, t87797: f64, t87805: f64, t87806: f64, t87807: f64, t87827: f64, t87836: f64, t87837: f64, t87847: f64, t87880: f64, t9590: f64, t870: f64, t1877: f64, t1915: f64, t22959: f64, t23290: f64, t25: f64, t25013: f64, t25021: f64, t25024: f64, t2522: f64, t25377: f64, t25381: f64, t25392: f64, t4314: f64, t6666: f64, t6670: f64, t6671: f64, t81483: f64, t86803: f64, t86806: f64, t86810: f64, t86816: f64, t86821: f64, t86825: f64, t86830: f64, t86835: f64, t86836: f64, t1484: f64, t2249: f64, t606: f64, t1408: f64, t2749: f64, t10143: f64, t7540: f64, t13191: f64, t25014: f64, t13196: f64, t13471: f64, t25373: f64, t57921: f64, t1530: f64, t16596: f64, t81547: f64, t22951: f64, t22968: f64, t23295: f64, t23296: f64, t23302: f64, t25354: f64, t25358: f64, t6542: f64, t7541: f64, t86752: f64, t86801: f64, t12606: f64, t3: f64, t12915: f64, t13487: f64, t193: f64, t202: f64, t2379: f64, t25365: f64, t25374: f64, t57893: f64, t57912: f64, t81525: f64, t81539: f64, t82312: f64, t86717: f64, t868: f64, t12971: f64, t23286: f64, t2745: f64, t4255: f64, t4303: f64, t47645: f64, t58009: f64, t58071: f64, t59580: f64, t7634: f64, t86706: f64, t86713: f64, t86815: f64, t23788: f64, t25891: f64, t25927: f64, t1081: f64, t1649: f64, t23789: f64, t23792: f64, t25372: f64, t6848: f64, t86736: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t87893, t87898, t87902, t87904, t87907) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2158(t1880, t23196, t25224, t23030, t25205, t23164, t7479, t82133, t6552, t82124, t23237, t25341);
        let (t87911, t87915, t87920) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2159(t23204, t25216, t6562, t1519, t212, t23171, t6554, t23270, t25038, t258, t4119, t776);
        let t87940 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2160(t23270, t25038, t25039, t2553, t25040, t82074, t87712, t82294, t25193, t81591, t10049, t13053, t6632, t6663, t7538, t82296, t87915, t87920, t9593);
        let t87944 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2161(t10049, t10109, t10110, t13029, t13042, t13050, t13058, t13071, t13460, t13463, t1528, t1902, t1911, t1912, t22979, t23191, t23215, t23278, t23281, t25168, t25169, t25170, t25188, t25200, t25233, t25329, t25330, t25348, t259, t2597, t2713, t2718, t2719, t2720, t2743, t4142, t4147, t4268, t4272, t4273, t4301, t47568, t47609, t6624, t6627, t6632, t6662, t7517, t7537, t7538, t82070, t82071, t82076, t82079, t82131, t82135, t82197, t82287, t855, t865, t866, t86905, t86909, t86911, t86916, t86923, t86952, t86955, t86961, t86968, t86972, t87005, t87010, t87013, t87741, t87792, t87797, t87805, t87806, t87807, t87827, t87836, t87837, t87847, t87880, t87893, t87898, t87902, t87904, t87907, t87911, t87940, t9590);
        let (t87945, t87952) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2162(t870, t87944, t1877, t1915, t22959, t23290, t25, t25013, t25021, t25024, t2522, t25377, t25381, t25392, t4314, t6666, t6670, t6671, t81483, t86803, t86806, t86810, t86816, t86821, t86825, t86830, t86835, t86836);
        let (t87953, t87957, t87961, t87975, t87978, t87981, t87984) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2163(t1484, t2249, t4119, t606, t1408, t2749, t10143, t7540, t13191, t25014, t13196, t13471, t25);
        let t88001 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2164(t25373, t57921, t1530, t2249, t16596, t81547, t1877, t1915, t22951, t22959, t22968, t23295, t23296, t23302, t25013, t2522, t25354, t25358, t4314, t606, t6542, t6670, t7541, t87953, t87957, t87961, t87975, t87978, t87981, t87984);
        let (t88003, t88391, t89775) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2165(t86752, t86801, t87952, t88001, t12606, t3, t12915, t13487, t13191, t13471, t1530, t16596, t1877, t1915, t193, t202, t22959, t23290, t2379, t25013, t2522, t25358, t25365, t25374, t2553, t4119, t4314, t57893, t57912, t6666, t6670, t7541, t81525, t81539, t82312, t86717, t868, t86836, t870, t87944);
        let t89822 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2166(t12971, t13196, t1484, t1877, t1915, t23286, t23290, t23295, t2522, t25354, t25358, t2745, t2749, t4255, t4303, t4314, t47645, t57921, t58009, t58071, t59580, t6666, t6670, t7634, t776, t86706, t86713, t86815, t87975);
        let (t89823, t89837, t89840, t89843, t89846, t89850) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2167(t89775, t89822, t23788, t59580, t86815, t13196, t25891, t25927, t58009, t10143, t1081, t25374);
        let t89880 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2168(t1081, t4255, t870, t23788, t58071, t86706, t1649, t2745, t25927, t86713, t2379, t1877, t1915, t22959, t23789, t23792, t25013, t2522, t25372, t4314, t6670, t6848, t7541, t86736, t86836, t89837, t89840, t89843, t89846, t89850);
    (t87945, t87975, t88003, t88391, t89823, t89880)
}
