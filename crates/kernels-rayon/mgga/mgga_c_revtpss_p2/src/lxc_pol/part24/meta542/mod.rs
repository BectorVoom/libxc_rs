//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta542 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1593;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1594;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1595;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1596;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1597;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1598;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1599;
use chunk7::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1600;
use chunk8::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1601;
use chunk9::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1602;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta542(t45927: f64, t45929: f64, t45931: f64, t45933: f64, t45935: f64, t45937: f64, t45939: f64, t45941: f64, t45944: f64, t45946: f64, t45948: f64, t45950: f64, t45952: f64, t5816: f64, t5872: f64, t5825: f64, t87028: f64, t30: f64, t33: f64, zeta_threshold: f64, t1800: f64, t53: f64, t5819: f64, sigma2: f64, t1480: f64, t1483: f64, t21732: f64, t21754: f64, t22671: f64, t22700: f64, t22709: f64, t22712: f64, t22715: f64, t2275: f64, t2282: f64, t4201: f64, t4210: f64, t44: f64, t46065: f64, t46074: f64, t46090: f64, t48: f64, t56: f64, t5843: f64, t5848: f64, t5851: f64, t60: f64, t61: f64, t1487: f64, t1494: f64, t21686: f64, t21784: f64, t21794: f64, t22662: f64, t22665: f64, t22719: f64, t22739: f64, t2299: f64, t2306: f64, t38: f64, t4227: f64, t4232: f64, t46001: f64, t46014: f64, t5820: f64, t5854: f64, t5855: f64, t5869: f64, t633: f64, t637: f64, t70: f64, t71: f64, t77: f64, t7719: f64, t85: f64, t85161: f64, t1470: f64, t1471: f64, t1486: f64, t1927: f64, t22672: f64, t22673: f64, t22676: f64, t22681: f64, t22718: f64, t36: f64, t5826: f64, t5827: f64, t5830: f64, t10309: f64, t13272: f64, t1497: f64, t21663: f64, t2247: f64, t22656: f64, t22659: f64, t22742: f64, t4173: f64, t45972: f64, t60224: f64, t603: f64, t60673: f64, t85037: f64, t91: f64, t5: f64, t117: f64, t5920: f64, t190: f64, t706: f64, t76892: f64, t23221: f64, t4311: f64, t1522: f64, t77054: f64, t49866: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t87072 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1593(t45927, t45929, t45931, t45933, t45935, t45937, t45939, t45941, t45944, t45946, t45948, t45950, t45952);
        let (t87086, t87092, t87107) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1594(t5816, t5872, t5825);
        let t87125 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1595(t87028);
        let t87126 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1596(t30, t33, t87125, zeta_threshold);
        let (t87132, t87145) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1597(t1800, t53, t5819, sigma2);
        let t87155 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1598(t1480, t1483, t21732, t21754, t22671, t22700, t22709, t22712, t22715, t2275, t2282, t4201, t4210, t44, t46065, t46074, t46090, t48, t56, t5825, t5843, t5848, t5851, t60, t61, t87107, t87126, t87132, t87145);
        let t87195 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1599(t1487, t1494, t21686, t21784, t21794, t22662, t22665, t22671, t22719, t22739, t2299, t2306, t38, t4227, t4232, t46001, t46014, t5819, t5820, t5825, t5854, t5855, t5869, t633, t637, t70, t71, t77, t7719, t85, t85161, t87107, t87126, t87145, t87155);
        let t87221 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1600(t1470, t1471, t1486, t1494, t1927, t21686, t22671, t22672, t22673, t22676, t22681, t22718, t22739, t36, t5826, t5827, t5830, t5854, t5869, t70, t85, t87126);
        let t87225 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1601(t10309, t13272, t1497, t21663, t2247, t22656, t22659, t22742, t4173, t45972, t5816, t5872, t60224, t603, t60673, t85037, t87072, t87086, t87092, t87195, t87221, t91);
        let (t87227, t87237, t87262, t87263, t87265, t87267, t87268) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1602(t5, t87225, t117, t5920, t190, t706, t87126, t76892, t23221, t4311, t1522, t77054, t49866);
    (t87107, t87125, t87126, t87132, t87145, t87227, t87237, t87262, t87263, t87265, t87267, t87268)
}
