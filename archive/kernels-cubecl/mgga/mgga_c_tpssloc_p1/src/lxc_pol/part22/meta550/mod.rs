//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta550 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2050;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta550<F: Float>(t39264: F, t761: F, t2531: F, t9905: F, t39259: F, t39358: F, t756: F, t187: F, t268: F, t39322: F, t39347: F, t39336: F) -> (F, F, F, F, F, F, F) {
        let (t40679, t40682, t40685, t40708, t40714, t40716, t40721) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2050::<F>(t39264, t761, t2531, t9905, t39259, t39358, t756, t187, t268, t39322, t39347, t39336);
    (t40679, t40682, t40685, t40708, t40714, t40716, t40721)
}
