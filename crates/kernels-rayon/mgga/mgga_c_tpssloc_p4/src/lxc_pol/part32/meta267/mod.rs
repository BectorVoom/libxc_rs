//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta267 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1210;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1211;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1212;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1213;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1214;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1215;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1216;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta267(t1170: f64, t2148: f64, t2121: f64, t225: f64, t7284: f64, t477: f64, t491: f64, t1090: f64, t1186: f64, t50: f64, t6794: f64, t131: f64, t467: f64, t1009: f64, t461: f64, t1209: f64, t475: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7359, t7361, t7362) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1210(t1170, t2148, t2121, t225, t7284);
        let t7363 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1211(t477, t491);
        let (t7364, t7365) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1212(t1090, t7363, t7362);
        let t7368 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1213(t1186, t2148);
        let (t7371, t7372, t7373) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1214(t50, t6794, t131, t467);
        let t7375 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1215(t1009, t461, t1209);
        let t7376 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1216(t475, t68);
    (t7359, t7361, t7362, t7363, t7364, t7365, t7368, t7371, t7372, t7373, t7375, t7376)
}
