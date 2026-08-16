//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta34 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk248;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk249;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk250;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk251;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk252;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta34(t111: f64, t89: f64, t107: f64, t626: f64, t106: f64, t38: f64, t606: f64, tau0: f64, t95: f64, t103: f64, t100: f64, t92: f64, t96: f64, t109: f64, t64: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t652 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk248(t111, t89);
        let (t654, t655, t656) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk249(t107, t626, t106);
        let (t657, t659) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk250(t38, t606, tau0);
        let (t660, t662, t663, t666) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk251(t659, t95, t103, t100, t657, t92, t96);
        let (t667, t671) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk252(t109, t656, t666, t64, t654);
    (t652, t654, t655, t656, t657, t659, t660, t662, t663, t666, t667, t671)
}
