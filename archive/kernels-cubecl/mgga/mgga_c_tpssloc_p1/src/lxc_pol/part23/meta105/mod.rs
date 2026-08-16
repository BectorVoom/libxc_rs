//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta105 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk577;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk578;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta105<F: Float>(t1336: F, t3865: F, t1995: F, t241: F, t67: F, t1376: F, t566: F, t68: F, t3787: F, t562: F, t193: F, t532: F) -> (F, F, F, F, F) {
        let t3866 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk577::<F>(t1336, t3865);
        let (t3870, t3887, t3897, t3918) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk578::<F>(t1995, t241, t67, t1376, t566, t68, t3787, t562, t193, t532);
    (t3866, t3870, t3887, t3897, t3918)
}
