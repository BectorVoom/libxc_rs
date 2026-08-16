//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta39 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk273;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk274;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk275;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk276;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk277;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta39<F: Float>(t153: F, t751: F, t157: F, t717: F, t182: F, t187: F, t67: F, t181: F, t676: F, t686: F, t172: F, t739: F, t745: F, t746: F, t201: F, t262: F, t73: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t752, t753, t755, t756, t758) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk273::<F>(t153, t751, t157, t717, t182, t187, t67, t181, t676, t686);
        let (t760, t761) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk274::<F>(t756, t758, t172, t187);
        let t763 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk275::<F>(t739, t745, t746);
        let (t765, t766) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk276::<F>(t761, t763, t201, t262);
        let t767 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk277::<F>(t73);
    (t752, t753, t755, t756, t758, t760, t761, t763, t765, t766, t767)
}
