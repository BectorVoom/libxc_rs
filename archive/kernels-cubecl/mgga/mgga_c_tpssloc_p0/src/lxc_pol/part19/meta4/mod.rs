//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta4 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk34;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk35;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk36;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk37;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta4<F: Float>(t68: F, t67: F, t40: F, t52: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t71, t72) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk34::<F>(t68, t67);
        let t73 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk35::<F>(t40);
        let (t74, t75, t76) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk36::<F>(t40, t73, t52);
        let (t77, t78, t79, t80) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk37::<F>(t52, t76, t75, t72);
    (t71, t72, t73, t74, t75, t76, t77, t78, t79, t80)
}
