//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta14 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk111;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk112;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk113;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk114;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk115;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk116;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk117;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk118;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk119;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta14(t201: f64, t132: f64, t67: f64, t120: f64, t242: f64, t219: f64, t222: f64, t238: f64, t218: f64, t225: f64, t68: f64, t235: f64, t226: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t243 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk111(t201);
        let t244 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk112(t243);
        let t246 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk113(t132);
        let (t247, t248) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk114(t246, t67, t120);
        let t249 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk115(t242, t244, t248);
        let t252 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk116(t219, t222, t238, t249);
        let (t253, t254) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk117(t218, t252, t225, t68);
        let t255 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk118(t235, t252);
        let (t257, t259) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk119(t226, t255, t254);
    (t243, t244, t246, t247, t248, t249, t252, t253, t254, t255, t257, t259)
}
