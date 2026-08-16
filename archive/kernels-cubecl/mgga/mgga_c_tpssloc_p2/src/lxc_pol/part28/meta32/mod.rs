//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta32 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk224;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk225;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk226;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk227;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk228;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk229;
use chunk6::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk230;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta32<F: Float>(t24: F, t604: F, t4: F, t581: F, t25: F, t28: F, zeta_threshold: F, t31: F, t65: F, t34: F, t36: F, rho0: F, sigma0: F, t43: F, t55: F, t583: F, t61: F, t59: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t605 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk224::<F>(t24, t604);
        let t606 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk225::<F>(t4, t581);
        let t607 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk226::<F>(t25, t28, t606, zeta_threshold);
        let t608 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk227::<F>(t31, t607);
        let (t609, t612, t614, t615) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk228::<F>(t608, t65, t34, t36, rho0, sigma0);
        let (t618, t621, t625) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk229::<F>(t43, t607, t55, t583, t61);
        let t626 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk230::<F>(t59, t625);
    (t605, t606, t607, t608, t609, t612, t614, t615, t618, t621, t625, t626)
}
