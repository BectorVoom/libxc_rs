//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta640 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2180;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta640<F: Float>(t16081: F, t19787: F, t20032: F, t225: F, t20040: F, t19635: F, t20048: F, t16398: F, t20004: F, t19945: F, t19966: F, t5259: F, t53945: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t56550, t56580, t56596, t56607, t56640, t56685, t56687, t56693, t56710) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2180::<F>(t16081, t19787, t20032, t225, t20040, t19635, t20048, t16398, t20004, t19945, t19966, t5259, t53945);
    (t56550, t56580, t56596, t56607, t56640, t56685, t56687, t56693, t56710)
}
