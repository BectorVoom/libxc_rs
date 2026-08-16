//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta217 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1249;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1250;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1251;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1252;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1253;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1254;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1255;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1256;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta217(t3805: f64, t5249: f64, t6394: f64, t5264: f64, t5266: f64, t2408: f64, t2417: f64, t2426: f64, t2486: f64, t3688: f64, t3813: f64, t6299: f64, t6304: f64, t6329: f64, t2423: f64, t3686: f64, t3690: f64, t3695: f64, t3819: f64, t3821: f64, t3823: f64, t3825: f64, t3832: f64, t3836: f64, t6300: f64, t6322: f64, t225: f64, t3843: f64, t6330: f64, t1347: f64, t6347: f64, t1819: f64, t1821: f64, t546: f64, t548: f64, t550: f64, t1343: f64, t820: f64, t6387: f64, t3870: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t6396 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1249(t3805, t5249, t6394);
        let (t6399, t6400, t6401, t6402) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1250(t5264, t5266, t2408, t2417, t2426, t2486, t3688, t3813, t6299, t6304, t6329, t2423, t3686, t3690, t3695, t3819, t3821, t3823, t3825, t3832, t3836, t6300, t6322);
        let (t6404, t6408, t6411, t6414) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1251(t225, t6401, t6402, t3843, t6330, t1347, t6347, t1819, t1821, t546, t548);
        let t6415 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1252(t550, t6414);
        let t6417 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1253(t1343, t6415, t820);
        let t6420 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1254(t550, t6387);
        let t6422 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1255(t1343, t6420, t820);
        let t6427 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1256(t3870, t6330, t820);
    (t6396, t6399, t6400, t6404, t6408, t6411, t6414, t6415, t6417, t6420, t6422, t6427)
}
