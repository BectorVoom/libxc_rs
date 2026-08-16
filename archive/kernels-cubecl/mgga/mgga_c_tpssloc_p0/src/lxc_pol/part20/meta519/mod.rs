//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta519 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2047;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta519<F: Float>(t2225: F, t3824: F, t12129: F, t588: F, t39035: F, t522: F, t39031: F, t1285: F, t9216: F, t9218: F, t16: F, t185: F, t520: F) -> (F, F, F, F, F, F, F) {
        let (t39595, t39601, t39605, t39607, t39609, t39611, t39615) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2047::<F>(t2225, t3824, t12129, t588, t39035, t522, t39031, t1285, t9216, t9218, t16, t185, t520);
    (t39595, t39601, t39605, t39607, t39609, t39611, t39615)
}
