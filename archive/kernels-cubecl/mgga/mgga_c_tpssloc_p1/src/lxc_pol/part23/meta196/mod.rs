//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta196 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk834;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta196<F: Float>(t11135: F, t11203: F, t11153: F, t461: F, t1176: F, t698: F, t135: F, t3439: F, t3247: F, t405: F) -> (F, F, F, F, F, F) {
        let (t11459, t11487, t11516, t11529, t11539, t11545) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk834::<F>(t11135, t11203, t11153, t461, t1176, t698, t135, t3439, t3247, t405);
    (t11459, t11487, t11516, t11529, t11539, t11545)
}
