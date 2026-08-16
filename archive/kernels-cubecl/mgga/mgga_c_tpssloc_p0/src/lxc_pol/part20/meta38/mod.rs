//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta38 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk274;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk275;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk276;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk277;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk278;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk279;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk280;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta38<F: Float>(t180: F, t745: F, t118: F, t168: F, t181: F, t677: F, t680: F, t705: F, t725: F, t732: F, t740: F, t157: F, t153: F, t717: F, t182: F, t187: F, t67: F, t676: F, t686: F, t172: F, t739: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t746 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk274::<F>(t180);
        let t747 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk275::<F>(t745, t746);
        let t750 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk276::<F>(t118, t168, t181, t677, t680, t705, t725, t732, t740, t747);
        let t751 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk277::<F>(t157, t750);
        let (t752, t753, t755, t756, t758) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk278::<F>(t153, t751, t157, t717, t182, t187, t67, t181, t676, t686);
        let (t760, t761) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk279::<F>(t756, t758, t172, t187);
        let t763 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk280::<F>(t739, t745, t746);
    (t746, t747, t750, t751, t752, t753, t755, t756, t758, t760, t761, t763)
}
