//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta3 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk20;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk21;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk22;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk23;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta3<F: Float>(t41: F, t43: F, rho1: F, sigma2: F, t31: F, sigma0: F, sigma1: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t44, t46, t47, t48, t50, t51) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk20::<F>(t41, t43, rho1, sigma2);
        let t52 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk21::<F>(t31);
        let (t53, t54, t55) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk22::<F>(t52);
        let (t56, t59) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk23::<F>(t53, t55, sigma0, sigma1, sigma2);
    (t44, t46, t47, t48, t50, t51, t52, t53, t54, t55, t56, t59)
}
