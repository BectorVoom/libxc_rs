//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta32 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk224;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk225;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk226;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk227;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk228;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk229;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk230;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta32(t24: f64, t604: f64, t4: f64, t581: f64, t25: f64, t28: f64, zeta_threshold: f64, t31: f64, t65: f64, t34: f64, t36: f64, rho0: f64, sigma0: f64, t43: f64, t55: f64, t583: f64, t61: f64, t59: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t605 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk224(t24, t604);
        let t606 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk225(t4, t581);
        let t607 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk226(t25, t28, t606, zeta_threshold);
        let t608 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk227(t31, t607);
        let (t609, t615) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk228(t608, t65, t34, t36, rho0, sigma0);
        let (t618, t621, t625) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk229(t43, t607, t55, t583, t61);
        let t626 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk230(t59, t625);
    (t605, t606, t607, t608, t609, t615, t618, t621, t625, t626)
}
