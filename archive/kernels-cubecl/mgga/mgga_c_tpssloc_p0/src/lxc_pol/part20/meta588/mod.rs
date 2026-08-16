//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta588 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2166;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2167;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta588<F: Float>(t2296: F, t3241: F, t11778: F, t154: F, t1091: F, t9698: F, t22715: F, t268: F, t405: F) -> (F, F, F, F) {
        let (t43791, t43809, t43816) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2166::<F>(t2296, t3241, t11778, t154, t1091, t9698);
        let t43819 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2167::<F>(t22715, t268, t405);
    (t43791, t43809, t43816, t43819)
}
