//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta624 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2403;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2404;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta624<F: Float>(t12199: F, t12208: F, t3774: F, t3862: F, t241: F, t6597: F, t248: F, t555: F, t557: F, t3787: F, t3879: F, t12019: F, t566: F, t68: F, t3700: F, t2517: F, t2519: F, t195: F, t632: F, t197: F, t636: F, t2531: F, t9892: F, t718: F, t9862: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t40425, t40443, t40445, t40449, t40486, t40590) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2403::<F>(t12199, t12208, t3774, t3862, t241, t6597, t248, t555, t557, t3787, t3879, t12019, t566);
        let (t40591, t40611, t40626, t40632, t40647, t40667, t40673) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2404::<F>(t40590, t68, t3700, t2517, t2519, t195, t632, t197, t636, t2531, t9892, t718, t9862);
    (t40425, t40443, t40445, t40449, t40486, t40591, t40611, t40626, t40632, t40647, t40667, t40673)
}
