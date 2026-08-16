//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta39 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk281;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk282;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk283;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk284;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk285;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk286;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk287;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk288;
use chunk8::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk289;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta39<F: Float>(t761: F, t763: F, t201: F, t262: F, t73: F, t40: F, t607: F, t76: F, zeta_threshold: F, t52: F, t583: F, t60: F, t59: F, t207: F, t215: F, t154: F, t229: F, t205: F, t210: F, t214: F, t16: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t765, t766) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk281::<F>(t761, t763, t201, t262);
        let t767 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk282::<F>(t73);
        let (t770, t771) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk283::<F>(t40, t607, t767, t76, zeta_threshold);
        let t776 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk284::<F>(t52, t607, t771, t770, zeta_threshold);
        let t781 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk285::<F>(t583, t60);
        let t782 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk286::<F>(t59, t781);
        let (t785, t786) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk287::<F>(t207, t215, t782, t154, t229);
        let t787 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk288::<F>(t205, t786);
        let (t789, t792) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk289::<F>(t210, t214, t776, t16, t59);
    (t765, t766, t767, t771, t776, t781, t782, t785, t786, t787, t789, t792)
}
