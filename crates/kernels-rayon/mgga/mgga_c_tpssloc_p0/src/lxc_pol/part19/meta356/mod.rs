//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta356 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1287;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1288;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1289;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1290;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1291;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1292;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1293;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta356(t2793: f64, t10661: f64, t913: f64, t2836: f64, t2792: f64, t2842: f64, t2844: f64, t2880: f64, t2897: f64, t2904: f64, t10701: f64, t888: f64, t10705: f64, t275: f64, t2790: f64, t2840: f64, t10704: f64, t41642: f64, t41656: f64, t41658: f64, t41660: f64, t41662: f64, t41669: f64, t41673: f64, t41675: f64, t41831: f64, t41833: f64, t41836: f64, t41839: f64, t41842: f64, t41845: f64, t41678: f64, t41682: f64, t41684: f64, t41690: f64, t41699: f64, t41703: f64, t41711: f64, t41863: f64, t41865: f64, t41868: f64, t41870: f64, t41872: f64, t41874: f64, t41876: f64, t41646: f64, t41651: f64, t41680: f64, t41695: f64, t41707: f64, t41713: f64, t41717: f64, t41882: f64, t41885: f64, t41887: f64, t41889: f64, t41892: f64, t41927: f64, t41929: f64, t41654: f64, t41961: f64, t41937: f64, t41940: f64, t41943: f64, t41945: f64, t41948: f64, t41951: f64, t41954: f64, t41957: f64, t41964: f64, t41967: f64, t41970: f64, t41973: f64, t893: f64, t2843: f64, t10619: f64, t942: f64, t2928: f64, t315: f64, t2931: f64, t10843: f64, t923: f64, t10744: f64, t10750: f64, t10760: f64, t10765: f64, t10771: f64, t10825: f64, t2861: f64, t2881: f64, t2886: f64, t2888: f64, t2907: f64, t41827: f64, t41987: f64, t932: f64, t933: f64, t952: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41995, t41998, t42002, t42005, t42011, t42020, t42023) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1287(t2793, t10661, t913, t2836, t2792, t2842, t2844, t2880, t2897, t2904, t10701, t888);
        let (t42025, t42031, t42046) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1288(t10705, t42023, t275, t2790, t2840, t10704, t41995, t41642, t41656, t41658, t41660, t41662, t41669, t41673, t41675, t41831, t41833, t41836, t41839, t41842, t41845);
        let t42061 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1289(t41678, t41682, t41684, t41690, t41699, t41703, t41711, t41863, t41865, t41868, t41870, t41872, t41874, t41876);
        let t42077 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1290(t41646, t41651, t41680, t41695, t41707, t41713, t41717, t41882, t41885, t41887, t41889, t41892, t41927, t41929);
        let t42092 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1291(t41654, t41961, t41937, t41940, t41943, t41945, t41948, t41951, t41954, t41957, t41964, t41967, t41970, t41973);
        let (t42097, t42105, t42106) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1292(t42046, t42061, t42077, t42092, t893, t913, t2840, t275, t2843, t41995, t10619, t942);
        let (t42110, t42113, t42122) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1293(t2928, t315, t2931, t10843, t923, t10744, t10750, t10760, t10765, t10771, t10825, t2861, t2881, t2886, t2888, t2907, t41827, t41987, t41998, t42002, t42005, t42011, t42020, t42025, t42031, t42097, t42105, t42106, t932, t933, t952);
    (t41995, t41998, t42002, t42005, t42025, t42031, t42097, t42105, t42110, t42113, t42122)
}
