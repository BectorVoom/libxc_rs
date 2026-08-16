//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta368 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1168;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta368<F: Float>(t11147: F, t460: F, t11545: F, t135: F, t43791: F, t461: F, t3439: F, t698: F, t1176: F, t697: F, t11153: F, t3242: F, t405: F) -> (F, F, F, F, F, F, F) {
        let (t44505, t44562, t44566, t44571, t44583, t44607, t44620) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1168::<F>(t11147, t460, t11545, t135, t43791, t461, t3439, t698, t1176, t697, t11153, t3242, t405);
    (t44505, t44562, t44566, t44571, t44583, t44607, t44620)
}
