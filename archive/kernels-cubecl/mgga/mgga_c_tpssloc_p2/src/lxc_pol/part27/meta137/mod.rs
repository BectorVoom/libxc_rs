//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta137 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk781;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk782;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk783;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk784;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk785;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk786;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk787;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta137<F: Float>(t3030: F, t349: F, t1011: F, t68: F, t371: F, t335: F, t368: F, t1015: F, t1022: F, t360: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t3031, t3032) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk781::<F>(t3030, t349, t1011, t68);
        let t3033 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk782::<F>(t3031, t3032);
        let t3034 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk783::<F>(t371);
        let t3036 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk784::<F>(t3034, t335);
        let (t3037, t3038, t3039) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk785::<F>(t3036, t368, t1015, t3033);
        let t3040 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk786::<F>(t1022);
        let t3041 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk787::<F>(t3040, t360);
    (t3031, t3032, t3033, t3034, t3036, t3037, t3038, t3039, t3040, t3041)
}
