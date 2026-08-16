//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta11 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk81;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk82;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk83;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk84;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk85;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk86;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk87;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk88;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta11<F: Float>(t182: F, t187: F, t68: F, t147: F, t40: F, t52: F, t73: F, t76: F, zeta_threshold: F, t10: F, t60: F, t59: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t189, t191) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk81::<F>(t182, t187);
        let t193 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk82::<F>(t68, t191);
        let t194 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk83::<F>(t147);
        let (t195, t197, t200) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk84::<F>(t40, t52, t73, t194, t76, zeta_threshold);
        let t201 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk85::<F>(t200);
        let t202 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk86::<F>(t200, t201);
        let t204 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk87::<F>(t10, t60);
        let t205 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk88::<F>(t204, t59);
    (t189, t191, t193, t194, t195, t197, t200, t201, t202, t204, t205)
}
