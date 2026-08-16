//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta425 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1593;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1594;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1595;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1596;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1597;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1598;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1599;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1600;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta425<F: Float>(t1196: F, t12552: F, t3497: F, t43977: F, t12235: F, t3531: F, t43830: F, t43832: F, t43837: F, t43841: F, t43845: F, t43849: F, t43858: F, t43862: F, t43865: F, t43871: F, t43877: F, t43813: F, t43854: F, t43883: F, t43886: F, t43888: F, t43890: F, t43892: F, t43894: F, t43896: F, t43899: F, t43902: F, t43905: F, t448: F, t300: F, t1126: F, t12226: F, t12231: F, t3382: F, t3431: F, t408: F, t3385: F, t12230: F, t43762: F, t43769: F, t43771: F, t43773: F, t43779: F, t43781: F, t43783: F, t43785: F, t43787: F, t43791: F, t43795: F, t43799: F, t43802: F, t43804: F, t43816: F, t43808: F, t43810: F, t43823: F, t43826: F, t43828: F, t43909: F, t43911: F, t43914: F, t43917: F, t43920: F, t43923: F, t43926: F, t43928: F, t43947: F, t43950: F, t43953: F, t43955: F, t43957: F, t1131: F, t1150: F, t198: F, t336: F, t3801: F, t43750: F, t43757: F, t43759: F, t43761: F, t43965: F, t43970: F, t43971: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t43980, t43982, t43994) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1593::<F>(t1196, t12552, t3497, t43977, t12235, t3531, t43830, t43832, t43837, t43841, t43845, t43849, t43858, t43862, t43865, t43871, t43877);
        let t44007 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1594::<F>(t43813, t43854, t43883, t43886, t43888, t43890, t43892, t43894, t43896, t43899, t43902, t43905);
        let (t44009, t44011, t44014, t44018, t44021) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1595::<F>(t43994, t44007, t448, t300, t1126, t12226, t12231, t3382, t3431, t408, t3385, t12230);
        let t44036 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1596::<F>(t43762, t43769, t43771, t43773, t43779, t43781, t43783, t43785, t43787, t43791, t43795, t43799, t43802, t43804);
        let t44051 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1597::<F>(t43813, t43816, t43808, t43810, t43823, t43826, t43828, t43830, t43832, t43837, t43841, t43845, t43849, t43854);
        let t44067 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1598::<F>(t43858, t43862, t43865, t43871, t43877, t43883, t43909, t43911, t43914, t43917, t43920, t43923, t43926, t43928);
        let t44082 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1599::<F>(t43886, t43888, t43890, t43892, t43894, t43896, t43899, t43902, t43905, t43947, t43950, t43953, t43955, t43957);
        let (t44087, t44088) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1600::<F>(t1131, t1150, t44036, t44051, t44067, t44082, t198, t336, t3801, t43750, t43757, t43759, t43761, t43965, t43970, t43971, t43980, t43982, t44011, t44014, t44021);
    (t43980, t43982, t44009, t44011, t44014, t44018, t44021, t44087, t44088)
}
