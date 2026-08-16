//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta35 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk245;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk246;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk247;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk248;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk249;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta35(t3: f64, t60: f64, t120: f64, t118: f64, t142: f64, t138: f64, t125: f64, t126: f64, t67: f64, t117: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t676 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk245(t3, t60);
        let t677 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk246(t120, t676);
        let t680 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk247(t118, t142, t677);
        let (t681, t682, t683, t685, t686) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk248(t138, t125, t126, t67, t117, t120);
        let (t687, t688, t690) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk249(t676, t686, t685, t118, t677);
    (t676, t677, t680, t681, t682, t683, t685, t686, t687, t688, t690)
}
