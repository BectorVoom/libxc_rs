//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta643 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2189;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2190;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2191;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2192;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2193;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2194;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2195;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2196;
use chunk8::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2197;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta643(t1880: f64, t23196: f64, t25224: f64, t23030: f64, t25205: f64, t23164: f64, t7479: f64, t82133: f64, t6552: f64, t82124: f64, t23237: f64, t25341: f64, t23204: f64, t25216: f64, t6562: f64, t1519: f64, t212: f64, t23171: f64, t6554: f64, t23270: f64, t25038: f64, t258: f64, t4119: f64, t776: f64, t25039: f64, t2553: f64, t25040: f64, t82074: f64, t87712: f64, t82294: f64, t25193: f64, t81591: f64, t10049: f64, t13053: f64, t6632: f64, t6663: f64, t7538: f64, t82296: f64, t9593: f64, t10109: f64, t10110: f64, t13029: f64, t13042: f64, t13050: f64, t13058: f64, t13071: f64, t13460: f64, t13463: f64, t1528: f64, t1902: f64, t1911: f64, t1912: f64, t22979: f64, t23191: f64, t23215: f64, t23278: f64, t23281: f64, t25168: f64, t25169: f64, t25170: f64, t25188: f64, t25200: f64, t25233: f64, t25329: f64, t25330: f64, t25348: f64, t259: f64, t2597: f64, t2713: f64, t2718: f64, t2719: f64, t2720: f64, t2743: f64, t4142: f64, t4147: f64, t4268: f64, t4272: f64, t4273: f64, t4301: f64, t47568: f64, t47609: f64, t6624: f64, t6627: f64, t6662: f64, t7517: f64, t7537: f64, t82070: f64, t82071: f64, t82076: f64, t82079: f64, t82131: f64, t82135: f64, t82197: f64, t82287: f64, t855: f64, t865: f64, t866: f64, t86905: f64, t86909: f64, t86911: f64, t86916: f64, t86923: f64, t86952: f64, t86955: f64, t86961: f64, t86968: f64, t86972: f64, t87005: f64, t87010: f64, t87013: f64, t87741: f64, t87792: f64, t87797: f64, t87805: f64, t87806: f64, t87807: f64, t87827: f64, t87836: f64, t87837: f64, t87847: f64, t87880: f64, t9590: f64, t870: f64, t1877: f64, t1915: f64, t22959: f64, t23290: f64, t25: f64, t25013: f64, t25021: f64, t25024: f64, t2522: f64, t25377: f64, t25381: f64, t25392: f64, t4314: f64, t6666: f64, t6670: f64, t6671: f64, t81483: f64, t86803: f64, t86806: f64, t86810: f64, t86816: f64, t86821: f64, t86825: f64, t86830: f64, t86835: f64, t86836: f64, t1484: f64, t2249: f64, t606: f64, t1408: f64, t2749: f64, t10143: f64, t7540: f64, t13191: f64, t25014: f64, t13196: f64, t13471: f64, t25373: f64, t57921: f64, t1530: f64, t16596: f64, t81547: f64, t22951: f64, t22968: f64, t23295: f64, t23296: f64, t23302: f64, t25354: f64, t25358: f64, t6542: f64, t7541: f64, t86752: f64, t86801: f64, t25608: f64, t381: f64, t25428: f64, t6712: f64, t13797: f64, t1926: f64, t221: f64, t10216: f64, t387: f64, t10277: f64, t1625: f64, t225: f64, t344: f64, t12648: f64, t14165: f64, t1927: f64, t23327: f64, t23329: f64, t23332: f64, t23588: f64, t23594: f64, t23728: f64, t25416: f64, t25423: f64, t25425: f64, t25429: f64, t25431: f64, t25432: f64, t25442: f64, t25815: f64, t4548: f64, t6691: f64, t7553: f64, t82402: f64, t82417: f64, t82502: f64, t83352: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t87893, t87898, t87902, t87904, t87907) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2189(t1880, t23196, t25224, t23030, t25205, t23164, t7479, t82133, t6552, t82124, t23237, t25341);
        let (t87911, t87915, t87920) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2190(t23204, t25216, t6562, t1519, t212, t23171, t6554, t23270, t25038, t258, t4119, t776);
        let t87940 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2191(t23270, t25038, t25039, t2553, t25040, t82074, t87712, t82294, t25193, t81591, t10049, t13053, t6632, t6663, t7538, t82296, t87915, t87920, t9593);
        let t87944 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2192(t10049, t10109, t10110, t13029, t13042, t13050, t13058, t13071, t13460, t13463, t1528, t1902, t1911, t1912, t22979, t23191, t23215, t23278, t23281, t25168, t25169, t25170, t25188, t25200, t25233, t25329, t25330, t25348, t259, t2597, t2713, t2718, t2719, t2720, t2743, t4142, t4147, t4268, t4272, t4273, t4301, t47568, t47609, t6624, t6627, t6632, t6662, t7517, t7537, t7538, t82070, t82071, t82076, t82079, t82131, t82135, t82197, t82287, t855, t865, t866, t86905, t86909, t86911, t86916, t86923, t86952, t86955, t86961, t86968, t86972, t87005, t87010, t87013, t87741, t87792, t87797, t87805, t87806, t87807, t87827, t87836, t87837, t87847, t87880, t87893, t87898, t87902, t87904, t87907, t87911, t87940, t9590);
        let (t87945, t87952) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2193(t870, t87944, t1877, t1915, t22959, t23290, t25, t25013, t25021, t25024, t2522, t25377, t25381, t25392, t4314, t6666, t6670, t6671, t81483, t86803, t86806, t86810, t86816, t86821, t86825, t86830, t86835, t86836);
        let (t87953, t87957, t87961, t87975, t87978, t87981, t87984) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2194(t1484, t2249, t4119, t606, t1408, t2749, t10143, t7540, t13191, t25014, t13196, t13471, t25);
        let t88001 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2195(t25373, t57921, t1530, t2249, t16596, t81547, t1877, t1915, t22951, t22959, t22968, t23295, t23296, t23302, t25013, t2522, t25354, t25358, t4314, t606, t6542, t6670, t7541, t87953, t87957, t87961, t87975, t87978, t87981, t87984);
        let (t88003, t88004, t88016, t88022, t88023) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2196(t86752, t86801, t87952, t88001, t25608, t381, t25428, t6712, t13797, t1926, t221, t10216, t387);
        let t88054 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2197(t10277, t387, t1625, t225, t344, t12648, t14165, t1927, t23327, t23329, t23332, t23588, t23594, t23728, t25416, t25423, t25425, t25429, t25431, t25432, t25442, t25815, t4548, t6691, t7553, t82402, t82417, t82502, t83352, t88004, t88016, t88022, t88023);
    (t87944, t87945, t87975, t88003, t88016, t88022, t88054)
}
