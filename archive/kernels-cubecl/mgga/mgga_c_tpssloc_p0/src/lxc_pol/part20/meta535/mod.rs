//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta535 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2073;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2074;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta535<F: Float>(t241: F, t6597: F, t248: F, t555: F, t557: F, t12434: F, t1338: F, t12019: F, t566: F, t68: F, t3700: F, t10121: F, t870: F, t2517: F, t2519: F, t195: F, t632: F, t197: F, t636: F, t2531: F, t9892: F, t67: F, t758: F, t9915: F, t718: F, t9862: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t40445, t40449, t40479, t40591, t40611, t40622) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2073::<F>(t241, t6597, t248, t555, t557, t12434, t1338, t12019, t566, t68, t3700, t10121, t870);
        let (t40626, t40632, t40647, t40667, t40670, t40673) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2074::<F>(t2517, t2519, t195, t632, t197, t636, t2531, t9892, t67, t758, t9915, t718, t9862);
    (t40445, t40449, t40479, t40591, t40611, t40622, t40626, t40632, t40647, t40667, t40670, t40673)
}
