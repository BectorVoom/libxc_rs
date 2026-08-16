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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1593;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1594;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1595;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1596;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1597;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1598;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1599;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1600;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta425(t1196: f64, t12552: f64, t3497: f64, t43977: f64, t12235: f64, t3531: f64, t43830: f64, t43832: f64, t43837: f64, t43841: f64, t43845: f64, t43849: f64, t43858: f64, t43862: f64, t43865: f64, t43871: f64, t43877: f64, t43813: f64, t43854: f64, t43883: f64, t43886: f64, t43888: f64, t43890: f64, t43892: f64, t43894: f64, t43896: f64, t43899: f64, t43902: f64, t43905: f64, t448: f64, t300: f64, t1126: f64, t12226: f64, t12231: f64, t3382: f64, t3431: f64, t408: f64, t3385: f64, t12230: f64, t43762: f64, t43769: f64, t43771: f64, t43773: f64, t43779: f64, t43781: f64, t43783: f64, t43785: f64, t43787: f64, t43791: f64, t43795: f64, t43799: f64, t43802: f64, t43804: f64, t43816: f64, t43808: f64, t43810: f64, t43823: f64, t43826: f64, t43828: f64, t43909: f64, t43911: f64, t43914: f64, t43917: f64, t43920: f64, t43923: f64, t43926: f64, t43928: f64, t43947: f64, t43950: f64, t43953: f64, t43955: f64, t43957: f64, t1131: f64, t1150: f64, t198: f64, t336: f64, t3801: f64, t43750: f64, t43757: f64, t43759: f64, t43761: f64, t43965: f64, t43970: f64, t43971: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43980, t43982, t43994) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1593(t1196, t12552, t3497, t43977, t12235, t3531, t43830, t43832, t43837, t43841, t43845, t43849, t43858, t43862, t43865, t43871, t43877);
        let t44007 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1594(t43813, t43854, t43883, t43886, t43888, t43890, t43892, t43894, t43896, t43899, t43902, t43905);
        let (t44009, t44011, t44014, t44018, t44021) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1595(t43994, t44007, t448, t300, t1126, t12226, t12231, t3382, t3431, t408, t3385, t12230);
        let t44036 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1596(t43762, t43769, t43771, t43773, t43779, t43781, t43783, t43785, t43787, t43791, t43795, t43799, t43802, t43804);
        let t44051 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1597(t43813, t43816, t43808, t43810, t43823, t43826, t43828, t43830, t43832, t43837, t43841, t43845, t43849, t43854);
        let t44067 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1598(t43858, t43862, t43865, t43871, t43877, t43883, t43909, t43911, t43914, t43917, t43920, t43923, t43926, t43928);
        let t44082 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1599(t43886, t43888, t43890, t43892, t43894, t43896, t43899, t43902, t43905, t43947, t43950, t43953, t43955, t43957);
        let (t44087, t44088) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1600(t1131, t1150, t44036, t44051, t44067, t44082, t198, t336, t3801, t43750, t43757, t43759, t43761, t43965, t43970, t43971, t43980, t43982, t44011, t44014, t44021);
    (t43980, t43982, t44009, t44011, t44014, t44018, t44021, t44087, t44088)
}
