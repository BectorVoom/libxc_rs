//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta371 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1379;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta371<F: Float>(t11282: F, t1164: F, t3403: F, t43679: F, t11294: F, t3411: F, t11131: F, t3399: F, t3402: F, t11176: F, t300: F, t1166: F) -> (F, F, F, F, F, F, F) {
        let (t43683, t43685, t43687, t43689, t43692, t43695, t43702) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1379::<F>(t11282, t1164, t3403, t43679, t11294, t3411, t11131, t3399, t3402, t11176, t300, t1166);
    (t43683, t43685, t43687, t43689, t43692, t43695, t43702)
}
