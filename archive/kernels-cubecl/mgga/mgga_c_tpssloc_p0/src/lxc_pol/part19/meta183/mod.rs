//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta183 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk834;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta183<F: Float>(t2250: F, t751: F, t707: F, t2447: F, t706: F, t708: F, t157: F, t9448: F, t182: F, t2509: F, t746: F, t9490: F) -> (F, F, F, F, F, F, F) {
        let (t9909, t9911, t9912, t9914, t9915, t9917, t9919) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk834::<F>(t2250, t751, t707, t2447, t706, t708, t157, t9448, t182, t2509, t746, t9490);
    (t9909, t9911, t9912, t9914, t9915, t9917, t9919)
}
