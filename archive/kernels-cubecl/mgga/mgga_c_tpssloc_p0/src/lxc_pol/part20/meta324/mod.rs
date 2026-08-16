//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta324 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1597;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1598;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1599;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta324<F: Float>(t3585: F, t820: F, t1216: F, t3243: F, t1090: F, t3494: F, t3578: F, t10401: F, t3575: F, t3610: F) -> (F, F, F, F, F, F, F) {
        let t11668 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1597::<F>(t3585, t820);
        let (t11669, t11670, t11673, t11674, t11677) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1598::<F>(t1216, t3243, t11668, t1090, t3494, t3578, t10401, t3575);
        let t11678 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1599::<F>(t11677, t3610);
    (t11668, t11669, t11670, t11673, t11674, t11677, t11678)
}
