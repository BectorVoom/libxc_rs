//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta478 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1692;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta478<F: Float>(t1437: F, t1864: F, t1410: F, t2240: F, t4017: F, t71: F, t12568: F, t33: F, t3953: F, t608: F, t641: F, t72: F) -> (F, F, F, F, F, F, F) {
        let (t26012, t26016, t26024, t26028, t26055, t26062, t26063) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1692::<F>(t1437, t1864, t1410, t2240, t4017, t71, t12568, t33, t3953, t608, t641, t72);
    (t26012, t26016, t26024, t26028, t26055, t26062, t26063)
}
