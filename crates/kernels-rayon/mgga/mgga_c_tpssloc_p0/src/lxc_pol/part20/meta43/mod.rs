//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta43 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk317;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk318;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk319;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk320;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk321;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk322;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk323;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk324;
use chunk8::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk325;
use chunk9::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk326;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta43(t218: f64, t852: f64, t225: f64, t253: f64, t257: f64, t68: f64, t252: f64, t814: f64, t829: f64, t235: f64, t226: f64, t255: f64, t808: f64, t812: f64, t259: f64, t799: f64, t261: f64, t193: f64, t202: f64, t680: f64, t705: f64, t710: f64, t719: f64, t752: f64, t755: f64, t760: f64, t765: f64, t766: f64, t776: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t853, t855) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk317(t218, t852, t225, t253);
        let (t856, t858) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk318(t257, t68);
        let t860 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk319(t252, t814);
        let t861 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk320(t829, t860);
        let t863 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk321(t235, t852);
        let t865 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk322(t226, t255, t808, t812, t861, t863);
        let t866 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk323(t858, t865);
        let t868 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk324(t259, t799, t853, t855, t866);
        let t870 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk325(t261);
        let t873 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk326(t193, t202, t680, t705, t710, t719, t752, t755, t760, t765, t766, t776, t868, t870);
    (t853, t855, t856, t858, t860, t861, t863, t865, t866, t868, t870, t873)
}
