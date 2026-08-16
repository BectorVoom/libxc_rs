//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta550 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2095;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta550<F: Float>(t172: F, t763: F, t9915: F, t2371: F, t9716: F, t2447: F, t32: F, t9929: F, t9932: F, t31: F, t717: F, t607: F, t707: F, t9862: F) -> (F, F, F, F, F, F) {
        let (t41265, t41274, t41279, t41282, t41284, t41291) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2095::<F>(t172, t763, t9915, t2371, t9716, t2447, t32, t9929, t9932, t31, t717, t607, t707, t9862);
    (t41265, t41274, t41279, t41282, t41284, t41291)
}
