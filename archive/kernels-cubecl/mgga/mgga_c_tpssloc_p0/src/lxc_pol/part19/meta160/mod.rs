//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta160 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk777;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta160<F: Float>(t2359: F, t626: F, t655: F, t2332: F, t666: F, t2331: F, t2358: F, t2261: F, t93: F, t94: F, t2342: F, t659: F, tau0: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t9363, t9364, t9365, t9367, t9370, t9371, t9374, t9384, t9385) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk777::<F>(t2359, t626, t655, t2332, t666, t2331, t2358, t2261, t93, t94, t2342, t659, tau0);
    (t9363, t9364, t9365, t9367, t9370, t9371, t9374, t9384, t9385)
}
