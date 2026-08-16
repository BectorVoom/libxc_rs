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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta438(t43813: f64, t43816: f64, t43808: f64, t43810: f64, t43823: f64, t43826: f64, t43828: f64, t43830: f64, t43832: f64, t43837: f64, t43841: f64, t43845: f64, t43849: f64, t43854: f64, t43858: f64, t43862: f64, t43865: f64, t43871: f64, t43877: f64, t43883: f64, t43909: f64, t43911: f64, t43914: f64, t43917: f64, t43920: f64, t43923: f64, t43926: f64, t43928: f64, t43886: f64, t43888: f64, t43890: f64, t43892: f64, t43894: f64, t43896: f64, t43899: f64, t43902: f64, t43905: f64, t43947: f64, t43950: f64, t43953: f64, t43955: f64, t43957: f64, t3475: f64, t426: f64, t3478: f64, t1179: f64, t12378: f64, t3488: f64, t3520: f64, t1161: f64, t1169: f64, t1180: f64, t1188: f64, t1189: f64, t12465: f64, t12470: f64, t12472: f64, t12473: f64, t12476: f64, t12481: f64, t12486: f64, t12488: f64, t12494: f64, t12548: f64, t12553: f64, t3447: f64, t3479: f64, t3480: f64, t3491: f64, t3498: f64, t3516: f64, t3523: f64, t3524: f64, t43753: f64, t43961: f64, t45057: f64, t45061: f64, t45064: f64, t45075: f64, t45080: f64, t45085: f64, t45103: f64, t1175: f64, t12552: f64, t43752: f64, t439: f64, t1160: f64, t12408: f64, t3519: f64, t3522: f64, t3444: f64, t3451: f64, t1156: f64, t12428: f64, t3471: f64, t1170: f64, t12418: f64, t12423: f64, t12429: f64, t12431: f64, t12514: f64, t12555: f64, t12556: f64, t3452: f64, t3454: f64, t3472: f64, t3477: f64, t3496: f64, t3521: f64, t43750: f64, t43966: f64, t44014: f64, t44021: f64, t44087: f64, t12504: f64, t12511: f64, t435: f64, t44009: f64, t44096: f64, t44100: f64, t44103: f64, t44106: f64, t44108: f64, t44111: f64, t44114: f64, t45015: f64, t45023: f64, t45026: f64, t45029: f64, t45033: f64, t45037: f64, t45040: f64, t12361: f64, t12411: f64, t12547: f64, t1168: f64, t1187: f64, t12464: f64, t12491: f64, t12497: f64, t12501: f64, t12508: f64, t3453: f64, t3497: f64, t3515: f64, t43977: f64, t45043: f64, t45045: f64, t45048: f64, t45050: f64, t300: f64, t12596: f64, t3531: f64, t1196: f64, t12485: f64, t5206: f64, t12581: f64, t12592: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t45118 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1651(t43813, t43816, t43808, t43810, t43823, t43826, t43828, t43830, t43832, t43837, t43841, t43845, t43849, t43854);
        let t45134 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1652(t43858, t43862, t43865, t43871, t43877, t43883, t43909, t43911, t43914, t43917, t43920, t43923, t43926, t43928);
        let t45149 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1653(t43886, t43888, t43890, t43892, t43894, t43896, t43899, t43902, t43905, t43947, t43950, t43953, t43955, t43957);
        let t45173 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1654(t3475, t426, t3478, t1179, t12378, t3488, t3520, t1161, t1169, t1180, t1188, t1189, t12465, t12470, t12472, t12473, t12476, t12481, t12486, t12488, t12494, t12548, t12553, t3447, t3479, t3480, t3491, t3498, t3516, t3523, t3524, t43753, t43961, t45057, t45061, t45064, t45075, t45080, t45085, t45103, t45118, t45134, t45149);
        let (t45174, t45177, t45181, t45187, t45188, t45190, t45194, t45197) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1655(t1175, t12552, t43752, t439, t1160, t12408, t3519, t3522, t3444, t3451, t1156, t12428);
        let t45218 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1656(t3471, t1169, t1170, t1188, t12418, t12423, t12429, t12431, t12514, t12555, t12556, t3452, t3454, t3472, t3477, t3479, t3496, t3521, t3523, t43750, t43753, t43966, t44014, t44021, t44087, t45057, t45174, t45177, t45181, t45188, t45190, t45194, t45197);
        let (t45231, t45232) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1657(t43830, t43832, t43837, t43841, t43845, t43849, t43858, t43862, t43865, t43871, t43877, t43813);
        let t45244 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1658(t43854, t43883, t43886, t43888, t43890, t43892, t43894, t43896, t43899, t43902, t43905, t45232);
        let t45251 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1659(t12504, t12511, t435, t44009, t44096, t44100, t44103, t44106, t44108, t44111, t44114, t45015, t45023, t45026, t45029, t45033, t45037, t45040, t45231, t45244);
        let (t45282, t45293) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1660(t12361, t12411, t12547, t3523, t1168, t1187, t1189, t12423, t12429, t12464, t12465, t12470, t12472, t12481, t12486, t12491, t12497, t12501, t12508, t12553, t3452, t3453, t3454, t3471, t3477, t3479, t3480, t3496, t3497, t3498, t3515, t3521, t3524, t43977, t45043, t45045, t45048, t45050);
        let (t45296, t45298, t45302, t45306) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1661(t300, t45173, t45218, t45251, t45293, t12596, t3531, t1196, t12552, t3523, t43753, t1188, t12485);
        let (t45310, t45312, t45316, t45318, t45319) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1662(t1196, t12547, t3520, t5206, t12581, t3531, t43753, t45187, t45190, t12592, t12378, t300);
    (t45282, t45296, t45298, t45302, t45306, t45310, t45312, t45316, t45318, t45319)
}
