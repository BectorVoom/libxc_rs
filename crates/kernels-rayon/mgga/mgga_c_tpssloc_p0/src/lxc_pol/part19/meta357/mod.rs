//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta357 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1294;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1295;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1296;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1297;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1298;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta357(t2853: f64, t2885: f64, t10523: f64, t938: f64, t10660: f64, t888: f64, t10663: f64, t10702: f64, t2844: f64, t41995: f64, t10810: f64, t919: f64, t2859: f64, t2884: f64, t302: f64, t41642: f64, t41656: f64, t41658: f64, t41660: f64, t41662: f64, t41669: f64, t41673: f64, t41675: f64, t41831: f64, t41833: f64, t41836: f64, t41839: f64, t41842: f64, t41845: f64, t41678: f64, t41682: f64, t41684: f64, t41690: f64, t41699: f64, t41703: f64, t41711: f64, t41863: f64, t41865: f64, t41868: f64, t41870: f64, t41872: f64, t41874: f64, t41876: f64, t41646: f64, t41651: f64, t41680: f64, t41695: f64, t41707: f64, t41713: f64, t41717: f64, t41882: f64, t41885: f64, t41887: f64, t41889: f64, t41892: f64, t41927: f64, t41929: f64, t41654: f64, t41961: f64, t41937: f64, t41940: f64, t41943: f64, t41945: f64, t41948: f64, t41951: f64, t41954: f64, t41957: f64, t41964: f64, t41967: f64, t41970: f64, t41973: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42123, t42128, t42145, t42148, t42149) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1294(t2853, t2885, t10523, t938, t10660, t888, t10663, t10702, t2844, t41995, t10810, t919);
        let (t42154, t42172) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1295(t2859, t2884, t302, t41642, t41656, t41658, t41660, t41662, t41669, t41673, t41675, t41831, t41833, t41836, t41839, t41842, t41845);
        let t42187 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1296(t41678, t41682, t41684, t41690, t41699, t41703, t41711, t41863, t41865, t41868, t41870, t41872, t41874, t41876);
        let t42203 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1297(t41646, t41651, t41680, t41695, t41707, t41713, t41717, t41882, t41885, t41887, t41889, t41892, t41927, t41929);
        let t42218 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1298(t41654, t41961, t41937, t41940, t41943, t41945, t41948, t41951, t41954, t41957, t41964, t41967, t41970, t41973);
    (t42123, t42128, t42145, t42148, t42149, t42154, t42172, t42187, t42203, t42218)
}
