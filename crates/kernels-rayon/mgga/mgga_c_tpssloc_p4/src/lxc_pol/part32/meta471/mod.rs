//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta471 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1765;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1766;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1767;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1768;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta471(t225: f64, t7319: f64, t23598: f64, t50: f64, t131: f64, t467: f64, t3030: f64, t461: f64, t3502: f64, t1011: f64, t3508: f64, t1209: f64, t475: f64, t1193: f64, t7372: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24788, t24810, t24811, t24812) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1765(t225, t7319, t23598, t50, t131, t467);
        let t24813 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1766(t3030, t461);
        let (t24814, t24815) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1767(t24813, t3502, t1011, t3508);
        let (t24820, t24821, t24826) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1768(t1209, t24813, t1011, t475, t1193, t7372);
    (t24788, t24810, t24811, t24812, t24813, t24814, t24815, t24820, t24821, t24826)
}
