//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta1 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk8;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk9;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk10;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk11;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk12;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk13;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk14;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk15;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta1<F: Float>(t14: F, t9: F, t10: F, t15: F, t11: F, t17: F, t5: F, zeta_threshold: F, rho0: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t19, t20, t21) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk8::<F>(t14, t9, t10, t15);
        let t24 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk9::<F>(t11, t17, t19, t21, t9);
        let t25 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk10::<F>(t5);
        let (t27, t28) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk11::<F>(t25, t5, zeta_threshold);
        let t31 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk12::<F>(t25, t28, t27, t5, zeta_threshold);
        let t32 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk13::<F>(t31);
        let t33 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk14::<F>(t32);
        let t34 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk15::<F>(rho0);
    (t19, t20, t21, t24, t25, t28, t31, t32, t33, t34)
}
