//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta349 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1143;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1144;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta349<F: Float>(t40419: F, t535: F, t9538: F, t241: F, t6597: F, t248: F, t555: F, t557: F, t40041: F, t562: F, t12019: F, t566: F, t68: F, t3700: F, t195: F, t632: F, t197: F, t636: F, t39264: F, t761: F, t39259: F, t39358: F, t756: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t40422, t40445, t40449, t40541, t40590) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1143::<F>(t40419, t535, t9538, t241, t6597, t248, t555, t557, t40041, t562, t12019, t566);
        let (t40591, t40611, t40632, t40647, t40679, t40685, t40708) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1144::<F>(t40590, t68, t3700, t195, t632, t197, t636, t39264, t761, t39259, t39358, t756);
    (t40422, t40445, t40449, t40541, t40591, t40611, t40632, t40647, t40679, t40685, t40708)
}
