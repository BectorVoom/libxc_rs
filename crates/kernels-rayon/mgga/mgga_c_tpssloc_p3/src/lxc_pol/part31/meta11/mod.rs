//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta11 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk74;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk75;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk76;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk77;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk78;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk79;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk80;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta11(t153: f64, t185: f64, t152: f64, t157: f64, t182: f64, t68: f64, t147: f64, t40: f64, t52: f64, t73: f64, t76: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t186, t187, t189, t191) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk74(t153, t185, t152, t157, t182);
        let t192 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk75(t68);
        let t193 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk76(t191, t192);
        let t194 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk77(t147);
        let (t195, t197, t200) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk78(t40, t52, t73, t194, t76, zeta_threshold);
        let t201 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk79(t200);
        let t202 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk80(t200, t201);
    (t186, t187, t189, t191, t192, t193, t194, t195, t197, t200, t201, t202)
}
