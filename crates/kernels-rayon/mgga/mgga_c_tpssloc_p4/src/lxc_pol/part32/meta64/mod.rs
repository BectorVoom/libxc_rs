//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta64 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk419;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk420;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk421;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk422;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk423;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk424;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk425;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk426;
use chunk8::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk427;
use chunk9::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk428;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta64(t1207: f64, t1212: f64, t486: f64, t61: f64, t1096: f64, t1121: f64, t1161: f64, t1163: f64, t1168: f64, t475: f64, t248: f64, t122: f64, t374: f64, t485: f64, t372: f64, t483: f64, t479: f64, t471: f64, t404: f64, t415: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1213 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk419(t1207, t1212);
        let t1214 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk420(t486, t61);
        let t1215 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk421(t1096, t1121, t1161, t1163, t1168);
        let t1216 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk422(t1215, t475);
        let t1218 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk423(t1214, t1216, t248);
        let t1222 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk424(t122, t374, t486);
        let (t1224, t1225, t1226) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk425(t1222, t485, t372, t483, t479);
        let t1227 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk426(t1226, t471);
        let t1229 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk427(t404, t415);
        let t1230 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk428(t1229, t61);
    (t1213, t1214, t1215, t1216, t1218, t1222, t1224, t1225, t1226, t1227, t1229, t1230)
}
