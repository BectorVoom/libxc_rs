//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta4 (260520-c91 hierarchical CSE).
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
mod chunk9;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk27;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk28;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk29;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk30;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk31;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk32;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk33;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk34;
use chunk8::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk35;
use chunk9::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk36;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta4<F: Float>(t10: F, t61: F, t59: F, t39: F, t44: F, t51: F, t56: F, t33: F, t40: F, t52: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t63 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk27::<F>(t10, t61);
        let t64 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk28::<F>(t59, t63);
        let t65 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk29::<F>(t39, t44, t51, t56, t64);
        let t66 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk30::<F>(t33, t65);
        let t67 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk31::<F>();
        let t68 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk32::<F>();
        let t71 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk33::<F>(t68);
        let t72 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk34::<F>(t67, t71);
        let t73 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk35::<F>(t40);
        let (t74, t75, t76) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk36::<F>(t40, t73, t52);
    (t63, t64, t65, t66, t67, t68, t71, t72, t73, t74, t75, t76)
}
