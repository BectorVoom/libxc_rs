//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta438 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1651;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1652;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1653;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1654;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1655;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1656;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1657;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1658;
use chunk8::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1659;
use chunk9::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1660;
use chunk10::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1661;
use chunk11::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1662;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta438<F: Float>(t43813: F, t43816: F, t43808: F, t43810: F, t43823: F, t43826: F, t43828: F, t43830: F, t43832: F, t43837: F, t43841: F, t43845: F, t43849: F, t43854: F, t43858: F, t43862: F, t43865: F, t43871: F, t43877: F, t43883: F, t43909: F, t43911: F, t43914: F, t43917: F, t43920: F, t43923: F, t43926: F, t43928: F, t43886: F, t43888: F, t43890: F, t43892: F, t43894: F, t43896: F, t43899: F, t43902: F, t43905: F, t43947: F, t43950: F, t43953: F, t43955: F, t43957: F, t3475: F, t426: F, t3478: F, t1179: F, t12378: F, t3488: F, t3520: F, t1161: F, t1169: F, t1180: F, t1188: F, t1189: F, t12465: F, t12470: F, t12472: F, t12473: F, t12476: F, t12481: F, t12486: F, t12488: F, t12494: F, t12548: F, t12553: F, t3447: F, t3479: F, t3480: F, t3491: F, t3498: F, t3516: F, t3523: F, t3524: F, t43753: F, t43961: F, t45057: F, t45061: F, t45064: F, t45075: F, t45080: F, t45085: F, t45103: F, t1175: F, t12552: F, t43752: F, t439: F, t1160: F, t12408: F, t3519: F, t3522: F, t3444: F, t3451: F, t1156: F, t12428: F, t3471: F, t1170: F, t12418: F, t12423: F, t12429: F, t12431: F, t12514: F, t12555: F, t12556: F, t3452: F, t3454: F, t3472: F, t3477: F, t3496: F, t3521: F, t43750: F, t43966: F, t44014: F, t44021: F, t44087: F, t12504: F, t12511: F, t435: F, t44009: F, t44096: F, t44100: F, t44103: F, t44106: F, t44108: F, t44111: F, t44114: F, t45015: F, t45023: F, t45026: F, t45029: F, t45033: F, t45037: F, t45040: F, t12361: F, t12411: F, t12547: F, t1168: F, t1187: F, t12464: F, t12491: F, t12497: F, t12501: F, t12508: F, t3453: F, t3497: F, t3515: F, t43977: F, t45043: F, t45045: F, t45048: F, t45050: F, t300: F, t12596: F, t3531: F, t1196: F, t12485: F, t5206: F, t12581: F, t12592: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t45118 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1651::<F>(t43813, t43816, t43808, t43810, t43823, t43826, t43828, t43830, t43832, t43837, t43841, t43845, t43849, t43854);
        let t45134 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1652::<F>(t43858, t43862, t43865, t43871, t43877, t43883, t43909, t43911, t43914, t43917, t43920, t43923, t43926, t43928);
        let t45149 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1653::<F>(t43886, t43888, t43890, t43892, t43894, t43896, t43899, t43902, t43905, t43947, t43950, t43953, t43955, t43957);
        let t45173 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1654::<F>(t3475, t426, t3478, t1179, t12378, t3488, t3520, t1161, t1169, t1180, t1188, t1189, t12465, t12470, t12472, t12473, t12476, t12481, t12486, t12488, t12494, t12548, t12553, t3447, t3479, t3480, t3491, t3498, t3516, t3523, t3524, t43753, t43961, t45057, t45061, t45064, t45075, t45080, t45085, t45103, t45118, t45134, t45149);
        let (t45174, t45177, t45181, t45187, t45188, t45190, t45194, t45197) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1655::<F>(t1175, t12552, t43752, t439, t1160, t12408, t3519, t3522, t3444, t3451, t1156, t12428);
        let t45218 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1656::<F>(t3471, t1169, t1170, t1188, t12418, t12423, t12429, t12431, t12514, t12555, t12556, t3452, t3454, t3472, t3477, t3479, t3496, t3521, t3523, t43750, t43753, t43966, t44014, t44021, t44087, t45057, t45174, t45177, t45181, t45188, t45190, t45194, t45197);
        let (t45231, t45232) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1657::<F>(t43830, t43832, t43837, t43841, t43845, t43849, t43858, t43862, t43865, t43871, t43877, t43813);
        let t45244 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1658::<F>(t43854, t43883, t43886, t43888, t43890, t43892, t43894, t43896, t43899, t43902, t43905, t45232);
        let t45251 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1659::<F>(t12504, t12511, t435, t44009, t44096, t44100, t44103, t44106, t44108, t44111, t44114, t45015, t45023, t45026, t45029, t45033, t45037, t45040, t45231, t45244);
        let (t45282, t45293) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1660::<F>(t12361, t12411, t12547, t3523, t1168, t1187, t1189, t12423, t12429, t12464, t12465, t12470, t12472, t12481, t12486, t12491, t12497, t12501, t12508, t12553, t3452, t3453, t3454, t3471, t3477, t3479, t3480, t3496, t3497, t3498, t3515, t3521, t3524, t43977, t45043, t45045, t45048, t45050);
        let (t45296, t45298, t45302, t45306) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1661::<F>(t300, t45173, t45218, t45251, t45293, t12596, t3531, t1196, t12552, t3523, t43753, t1188, t12485);
        let (t45310, t45312, t45316, t45318, t45319) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1662::<F>(t1196, t12547, t3520, t5206, t12581, t3531, t43753, t45187, t45190, t12592, t12378, t300);
    (t45282, t45296, t45298, t45302, t45306, t45310, t45312, t45316, t45318, t45319)
}
