//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta397 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1873;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta397<F: Float>(t10817: F, t4359: F, t10655: F, t4400: F, t4396: F, t912: F, t2792: F, t1557: F, t2836: F, t2793: F, t4399: F, t10661: F) -> (F, F, F, F, F, F, F, F) {
        let (t14376, t14378, t14379, t14381, t14382, t14384, t14385, t14387) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1873::<F>(t10817, t4359, t10655, t4400, t4396, t912, t2792, t1557, t2836, t2793, t4399, t10661);
    (t14376, t14378, t14379, t14381, t14382, t14384, t14385, t14387)
}
