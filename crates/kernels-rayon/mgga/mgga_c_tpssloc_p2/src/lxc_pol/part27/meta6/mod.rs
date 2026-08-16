//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta6 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk42;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk43;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk44;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk45;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk46;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta6(t88: f64, t36: f64, rho0: f64, tau0: f64, t25: f64, t48: f64, rho1: f64, tau1: f64, t28: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t89 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk42(t88);
        let t92 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk43(t36, rho0, tau0);
        let (t93, t94, t95) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk44(t25);
        let (t96, t100) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk45(t93, t95, t48, rho1, tau1);
        let (t101, t102, t103) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk46(t28);
    (t89, t92, t93, t94, t95, t96, t100, t101, t102, t103)
}
