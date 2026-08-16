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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta649<F: Float>(t1880: F, t23196: F, t25224: F, t23030: F, t25205: F, t23164: F, t7479: F, t82133: F, t6552: F, t82124: F, t23237: F, t25341: F, t23204: F, t25216: F, t6562: F, t1519: F, t212: F, t23171: F, t6554: F, t23270: F, t25038: F, t258: F, t4119: F, t776: F, t25039: F, t2553: F, t25040: F, t82074: F, t87712: F, t82294: F, t25193: F, t81591: F, t10049: F, t13053: F, t6632: F, t6663: F, t7538: F, t82296: F, t9593: F, t10109: F, t10110: F, t13029: F, t13042: F, t13050: F, t13058: F, t13071: F, t13460: F, t13463: F, t1528: F, t1902: F, t1911: F, t1912: F, t22979: F, t23191: F, t23215: F, t23278: F, t23281: F, t25168: F, t25169: F, t25170: F, t25188: F, t25200: F, t25233: F, t25329: F, t25330: F, t25348: F, t259: F, t2597: F, t2713: F, t2718: F, t2719: F, t2720: F, t2743: F, t4142: F, t4147: F, t4268: F, t4272: F, t4273: F, t4301: F, t47568: F, t47609: F, t6624: F, t6627: F, t6662: F, t7517: F, t7537: F, t82070: F, t82071: F, t82076: F, t82079: F, t82131: F, t82135: F, t82197: F, t82287: F, t855: F, t865: F, t866: F, t86905: F, t86909: F, t86911: F, t86916: F, t86923: F, t86952: F, t86955: F, t86961: F, t86968: F, t86972: F, t87005: F, t87010: F, t87013: F, t87741: F, t87792: F, t87797: F, t87805: F, t87806: F, t87807: F, t87827: F, t87836: F, t87837: F, t87847: F, t87880: F, t9590: F, t870: F, t1877: F, t1915: F, t22959: F, t23290: F, t25: F, t25013: F, t25021: F, t25024: F, t2522: F, t25377: F, t25381: F, t25392: F, t4314: F, t6666: F, t6670: F, t6671: F, t81483: F, t86803: F, t86806: F, t86810: F, t86816: F, t86821: F, t86825: F, t86830: F, t86835: F, t86836: F, t1484: F, t2249: F, t606: F, t1408: F, t2749: F, t10143: F, t7540: F, t13191: F, t25014: F, t13196: F, t13471: F, t25373: F, t57921: F, t1530: F, t16596: F, t81547: F, t22951: F, t22968: F, t23295: F, t23296: F, t23302: F, t25354: F, t25358: F, t6542: F, t7541: F, t86752: F, t86801: F, t12606: F, t3: F, t12915: F, t13487: F, t193: F, t202: F, t2379: F, t25365: F, t25374: F, t57893: F, t57912: F, t81525: F, t81539: F, t82312: F, t86717: F, t868: F, t12971: F, t23286: F, t2745: F, t4255: F, t4303: F, t47645: F, t58009: F, t58071: F, t59580: F, t7634: F, t86706: F, t86713: F, t86815: F, t23788: F, t25891: F, t25927: F, t1081: F, t1649: F, t23789: F, t23792: F, t25372: F, t6848: F, t86736: F) -> (F, F, F, F, F, F) {
        let (t87893, t87898, t87902, t87904, t87907) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2158::<F>(t1880, t23196, t25224, t23030, t25205, t23164, t7479, t82133, t6552, t82124, t23237, t25341);
        let (t87911, t87915, t87920) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2159::<F>(t23204, t25216, t6562, t1519, t212, t23171, t6554, t23270, t25038, t258, t4119, t776);
        let t87940 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2160::<F>(t23270, t25038, t25039, t2553, t25040, t82074, t87712, t82294, t25193, t81591, t10049, t13053, t6632, t6663, t7538, t82296, t87915, t87920, t9593);
        let t87944 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2161::<F>(t10049, t10109, t10110, t13029, t13042, t13050, t13058, t13071, t13460, t13463, t1528, t1902, t1911, t1912, t22979, t23191, t23215, t23278, t23281, t25168, t25169, t25170, t25188, t25200, t25233, t25329, t25330, t25348, t259, t2597, t2713, t2718, t2719, t2720, t2743, t4142, t4147, t4268, t4272, t4273, t4301, t47568, t47609, t6624, t6627, t6632, t6662, t7517, t7537, t7538, t82070, t82071, t82076, t82079, t82131, t82135, t82197, t82287, t855, t865, t866, t86905, t86909, t86911, t86916, t86923, t86952, t86955, t86961, t86968, t86972, t87005, t87010, t87013, t87741, t87792, t87797, t87805, t87806, t87807, t87827, t87836, t87837, t87847, t87880, t87893, t87898, t87902, t87904, t87907, t87911, t87940, t9590);
        let (t87945, t87952) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2162::<F>(t870, t87944, t1877, t1915, t22959, t23290, t25, t25013, t25021, t25024, t2522, t25377, t25381, t25392, t4314, t6666, t6670, t6671, t81483, t86803, t86806, t86810, t86816, t86821, t86825, t86830, t86835, t86836);
        let (t87953, t87957, t87961, t87975, t87978, t87981, t87984) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2163::<F>(t1484, t2249, t4119, t606, t1408, t2749, t10143, t7540, t13191, t25014, t13196, t13471, t25);
        let t88001 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2164::<F>(t25373, t57921, t1530, t2249, t16596, t81547, t1877, t1915, t22951, t22959, t22968, t23295, t23296, t23302, t25013, t2522, t25354, t25358, t4314, t606, t6542, t6670, t7541, t87953, t87957, t87961, t87975, t87978, t87981, t87984);
        let (t88003, t88391, t89775) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2165::<F>(t86752, t86801, t87952, t88001, t12606, t3, t12915, t13487, t13191, t13471, t1530, t16596, t1877, t1915, t193, t202, t22959, t23290, t2379, t25013, t2522, t25358, t25365, t25374, t2553, t4119, t4314, t57893, t57912, t6666, t6670, t7541, t81525, t81539, t82312, t86717, t868, t86836, t870, t87944);
        let t89822 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2166::<F>(t12971, t13196, t1484, t1877, t1915, t23286, t23290, t23295, t2522, t25354, t25358, t2745, t2749, t4255, t4303, t4314, t47645, t57921, t58009, t58071, t59580, t6666, t6670, t7634, t776, t86706, t86713, t86815, t87975);
        let (t89823, t89837, t89840, t89843, t89846, t89850) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2167::<F>(t89775, t89822, t23788, t59580, t86815, t13196, t25891, t25927, t58009, t10143, t1081, t25374);
        let t89880 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2168::<F>(t1081, t4255, t870, t23788, t58071, t86706, t1649, t2745, t25927, t86713, t2379, t1877, t1915, t22959, t23789, t23792, t25013, t2522, t25372, t4314, t6670, t6848, t7541, t86736, t86836, t89837, t89840, t89843, t89846, t89850);
    (t87945, t87975, t88003, t88391, t89823, t89880)
}
