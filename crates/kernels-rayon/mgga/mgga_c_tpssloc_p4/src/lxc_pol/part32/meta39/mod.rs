//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta39 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk272;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk273;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk274;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk275;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk276;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta39(t153: f64, t751: f64, t157: f64, t717: f64, t182: f64, t187: f64, t67: f64, t181: f64, t676: f64, t686: f64, t172: f64, t739: f64, t745: f64, t746: f64, t201: f64, t262: f64, t73: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t752, t753, t755, t756, t758) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk272(t153, t751, t157, t717, t182, t187, t67, t181, t676, t686);
        let (t760, t761) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk273(t756, t758, t172, t187);
        let t763 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk274(t739, t745, t746);
        let (t765, t766) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk275(t761, t763, t201, t262);
        let t767 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk276(t73);
    (t752, t753, t755, t756, t758, t760, t761, t763, t765, t766, t767)
}
