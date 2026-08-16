//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta701 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2265;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2266;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2267;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2268;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2269;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2270;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2271;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2272;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta701(t23164: f64, t7479: f64, t86893: f64, t17063: f64, t23278: f64, t25168: f64, t5637: f64, t82294: f64, t87748: f64, t87902: f64, t87911: f64, t87927: f64, t87932: f64, t92954: f64, t92961: f64, t99033: f64, t10109: f64, t13042: f64, t13463: f64, t1528: f64, t16804: f64, t17050: f64, t17057: f64, t17064: f64, t17069: f64, t17070: f64, t17090: f64, t17092: f64, t1902: f64, t1912: f64, t23281: f64, t25169: f64, t25170: f64, t25184: f64, t25188: f64, t25200: f64, t25233: f64, t25348: f64, t259: f64, t2713: f64, t2718: f64, t28307: f64, t28311: f64, t28431: f64, t4147: f64, t4268: f64, t4272: f64, t4273: f64, t4301: f64, t5558: f64, t5657: f64, t5658: f64, t59498: f64, t59503: f64, t59537: f64, t6624: f64, t6627: f64, t6632: f64, t6662: f64, t6663: f64, t7517: f64, t7537: f64, t7538: f64, t82087: f64, t82099: f64, t855: f64, t865: f64, t866: f64, t86903: f64, t86941: f64, t86943: f64, t87758: f64, t87777: f64, t87787: f64, t87797: f64, t87810: f64, t87836: f64, t87837: f64, t87874: f64, t92402: f64, t92863: f64, t98160: f64, t98164: f64, t98166: f64, t98172: f64, t98181: f64, t98208: f64, t98213: f64, t98222: f64, t98227: f64, t98258: f64, t98264: f64, t98277: f64, t98279: f64, t98309: f64, t98913: f64, t98921: f64, t98923: f64, t98947: f64, t98963: f64, t98966: f64, t98999: f64, t99003: f64, t99010: f64, t99019: f64, t99022: f64, t870: f64, t16596: f64, t86721: f64, t1484: f64, t584: f64, t86753: f64, t22959: f64, t16949: f64, t25014: f64, t1408: f64, t4255: f64, t193: f64, t200: f64, t7540: f64, t16557: f64, t1877: f64, t1915: f64, t23295: f64, t25: f64, t25013: f64, t25015: f64, t25021: f64, t2522: f64, t25354: f64, t25366: f64, t25372: f64, t25385: f64, t7541: f64, t86736: f64, t98091: f64, t98094: f64, t98103: f64, t98112: f64, t97989: f64, t98039: f64, t98090: f64, t1634: f64, t607: f64, t1065: f64, t5392: f64, t17686: f64, t1927: f64, t23327: f64, t23329: f64, t25424: f64, t25429: f64, t25430: f64, t25442: f64, t25738: f64, t25815: f64, t28701: f64, t28702: f64, t4337: f64, t7553: f64, t82342: f64, t82402: f64, t82417: f64, t88004: f64, t88050: f64, t88069: f64, t88075: f64, t88083: f64, t88089: f64, t88112: f64, t1625: f64, t7577: f64, t14552: f64, t1604: f64, t17691: f64, t254: f64, t25423: f64, t25431: f64, t25750: f64, t25759: f64, t25801: f64, t4342: f64, t6691: f64, t7625: f64, t82502: f64, t88058: f64, t88096: f64, t88162: f64, t23384: f64, t28481: f64, t14529: f64, t1539: f64, t17582: f64, t18061: f64, t1956: f64, t23346: f64, t23394: f64, t25784: f64, t28705: f64, t4548: f64, t4664: f64, t60971: f64, t61058: f64, t6687: f64, t6704: f64, t88100: f64, t88102: f64, t88152: f64, t88772: f64, t5837: f64, t984: f64, t28691: f64, t82431: f64, t14545: f64, t1635: f64, t18070: f64, t23336: f64, t23372: f64, t25420: f64, t25797: f64, t28491: f64, t4557: f64, t5944: f64, t61646: f64, t7565: f64, t7600: f64, t82481: f64, t88167: f64, t88194: f64, t88744: f64, t89598: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t99038 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2265(t23164, t7479, t86893, t17063, t23278, t25168, t5637, t82294, t87748, t87902, t87911, t87927, t87932, t92954, t92961, t99033);
        let t99042 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2266(t10109, t13042, t13463, t1528, t16804, t17050, t17057, t17064, t17069, t17070, t17090, t17092, t1902, t1912, t23278, t23281, t25168, t25169, t25170, t25184, t25188, t25200, t25233, t25348, t259, t2713, t2718, t28307, t28311, t28431, t4147, t4268, t4272, t4273, t4301, t5558, t5637, t5657, t5658, t59498, t59503, t59537, t6624, t6627, t6632, t6662, t6663, t7517, t7537, t7538, t82087, t82099, t855, t865, t866, t86903, t86941, t86943, t87758, t87777, t87787, t87797, t87810, t87836, t87837, t87874, t92402, t92863, t98160, t98164, t98166, t98172, t98181, t98208, t98213, t98222, t98227, t98258, t98264, t98277, t98279, t98309, t98913, t98921, t98923, t98947, t98963, t98966, t98999, t99003, t99010, t99019, t99022, t99038);
        let (t99043, t99049, t99055, t99056, t99060) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2267(t870, t99042, t16596, t86721, t1484, t584, t86753, t22959, t16949, t25014, t1408, t4255);
        let (t99064, t99067) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2268(t193, t200, t7540, t1408, t16557, t1877, t1915, t22959, t23295, t25, t25013, t25015, t25021, t2522, t25354, t25366, t25372, t25385, t7541, t86736, t98091, t98094, t98103, t98112, t99043, t99049, t99055, t99056, t99060);
        let (t99069, t99070, t99099, t99104) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2269(t97989, t98039, t98090, t99067, t1634, t607, t1065, t5392, t17686, t1927, t23327, t23329, t25424, t25429, t25430, t25442, t25738, t25815, t28701, t28702, t4337, t7553, t82342, t82402, t82417, t88004, t88050, t88069, t88075, t88083, t88089, t88112);
        let t99143 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2270(t1625, t7577, t14552, t1604, t17691, t23327, t23329, t254, t25423, t25424, t25429, t25431, t25442, t25750, t25759, t25801, t25815, t28701, t4342, t6691, t7553, t7625, t82502, t88050, t88058, t88096, t88112, t88162, t99070);
        let t99172 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2271(t23384, t28481, t14529, t1539, t17582, t18061, t1927, t1956, t23327, t23346, t23394, t25784, t28705, t4548, t4664, t60971, t61058, t6687, t6704, t7625, t82402, t88100, t88102, t88152, t88772);
        let (t99180, t99202) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2272(t5837, t984, t23384, t28691, t28705, t82431, t14545, t1635, t18070, t1956, t23327, t23336, t23372, t25420, t25429, t25750, t25797, t28491, t4557, t5944, t61646, t6687, t6704, t7565, t7600, t82481, t88162, t88167, t88194, t88744, t89598);
    (t99042, t99043, t99055, t99064, t99069, t99099, t99104, t99143, t99172, t99180, t99202)
}
