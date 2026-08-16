//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta262 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1011;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta262<F: Float>(t11720: F, t11721: F, t1214: F, t248: F, t11717: F, t3503: F, t11713: F, t3508: F, t11708: F, t3514: F, t1210: F, t475: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11722, t11724, t11727, t11728, t11729, t11731, t11734, t11737, t11738, t11739) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1011::<F>(t11720, t11721, t1214, t248, t11717, t3503, t11713, t3508, t11708, t3514, t1210, t475);
    (t11722, t11724, t11727, t11728, t11729, t11731, t11734, t11737, t11738, t11739)
}
