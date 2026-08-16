//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta226 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1283;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1284;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta226<F: Float>(t761: F, t9494: F, t116: F, t229: F, t212: F, t776: F, t2586: F, t597: F, t60: F, t59: F, t2386: F, t131: F, t207: F, t2559: F, t786: F) -> (F, F, F, F, F, F, F, F) {
        let (t9496, t9523, t9525, t9526, t9534, t9538) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1283::<F>(t761, t9494, t116, t229, t212, t776, t2586, t597, t60, t59, t2386, t131);
        let (t9540, t9541) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1284::<F>(t207, t9534, t9538, t2559, t786);
    (t9496, t9523, t9525, t9526, t9534, t9538, t9540, t9541)
}
