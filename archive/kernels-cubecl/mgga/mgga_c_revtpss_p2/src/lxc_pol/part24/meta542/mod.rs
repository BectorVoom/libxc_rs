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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta542<F: Float>(t45927: F, t45929: F, t45931: F, t45933: F, t45935: F, t45937: F, t45939: F, t45941: F, t45944: F, t45946: F, t45948: F, t45950: F, t45952: F, t5816: F, t5872: F, t5825: F, t87028: F, t30: F, t33: F, zeta_threshold: F, t1800: F, t53: F, t5819: F, sigma2: F, t1480: F, t1483: F, t21732: F, t21754: F, t22671: F, t22700: F, t22709: F, t22712: F, t22715: F, t2275: F, t2282: F, t4201: F, t4210: F, t44: F, t46065: F, t46074: F, t46090: F, t48: F, t56: F, t5843: F, t5848: F, t5851: F, t60: F, t61: F, t1487: F, t1494: F, t21686: F, t21784: F, t21794: F, t22662: F, t22665: F, t22719: F, t22739: F, t2299: F, t2306: F, t38: F, t4227: F, t4232: F, t46001: F, t46014: F, t5820: F, t5854: F, t5855: F, t5869: F, t633: F, t637: F, t70: F, t71: F, t77: F, t7719: F, t85: F, t85161: F, t1470: F, t1471: F, t1486: F, t1927: F, t22672: F, t22673: F, t22676: F, t22681: F, t22718: F, t36: F, t5826: F, t5827: F, t5830: F, t10309: F, t13272: F, t1497: F, t21663: F, t2247: F, t22656: F, t22659: F, t22742: F, t4173: F, t45972: F, t60224: F, t603: F, t60673: F, t85037: F, t91: F, t5: F, t117: F, t5920: F, t190: F, t706: F, t76892: F, t23221: F, t4311: F, t1522: F, t77054: F, t49866: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t87072 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1593::<F>(t45927, t45929, t45931, t45933, t45935, t45937, t45939, t45941, t45944, t45946, t45948, t45950, t45952);
        let (t87086, t87092, t87107) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1594::<F>(t5816, t5872, t5825);
        let t87125 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1595::<F>(t87028);
        let t87126 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1596::<F>(t30, t33, t87125, zeta_threshold);
        let (t87132, t87145) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1597::<F>(t1800, t53, t5819, sigma2);
        let t87155 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1598::<F>(t1480, t1483, t21732, t21754, t22671, t22700, t22709, t22712, t22715, t2275, t2282, t4201, t4210, t44, t46065, t46074, t46090, t48, t56, t5825, t5843, t5848, t5851, t60, t61, t87107, t87126, t87132, t87145);
        let t87195 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1599::<F>(t1487, t1494, t21686, t21784, t21794, t22662, t22665, t22671, t22719, t22739, t2299, t2306, t38, t4227, t4232, t46001, t46014, t5819, t5820, t5825, t5854, t5855, t5869, t633, t637, t70, t71, t77, t7719, t85, t85161, t87107, t87126, t87145, t87155);
        let t87221 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1600::<F>(t1470, t1471, t1486, t1494, t1927, t21686, t22671, t22672, t22673, t22676, t22681, t22718, t22739, t36, t5826, t5827, t5830, t5854, t5869, t70, t85, t87126);
        let t87225 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1601::<F>(t10309, t13272, t1497, t21663, t2247, t22656, t22659, t22742, t4173, t45972, t5816, t5872, t60224, t603, t60673, t85037, t87072, t87086, t87092, t87195, t87221, t91);
        let (t87227, t87237, t87262, t87263, t87265, t87267, t87268) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1602::<F>(t5, t87225, t117, t5920, t190, t706, t87126, t76892, t23221, t4311, t1522, t77054, t49866);
    (t87107, t87125, t87126, t87132, t87145, t87227, t87237, t87262, t87263, t87265, t87267, t87268)
}
