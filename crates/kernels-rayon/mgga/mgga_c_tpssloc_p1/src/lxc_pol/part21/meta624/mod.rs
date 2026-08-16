//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta624 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2403;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2404;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta624(t12199: f64, t12208: f64, t3774: f64, t3862: f64, t241: f64, t6597: f64, t248: f64, t555: f64, t557: f64, t3787: f64, t3879: f64, t12019: f64, t566: f64, t68: f64, t3700: f64, t2517: f64, t2519: f64, t195: f64, t632: f64, t197: f64, t636: f64, t2531: f64, t9892: f64, t718: f64, t9862: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40425, t40443, t40445, t40449, t40486, t40590) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2403(t12199, t12208, t3774, t3862, t241, t6597, t248, t555, t557, t3787, t3879, t12019, t566);
        let (t40591, t40611, t40626, t40632, t40647, t40667, t40673) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2404(t40590, t68, t3700, t2517, t2519, t195, t632, t197, t636, t2531, t9892, t718, t9862);
    (t40425, t40443, t40445, t40449, t40486, t40591, t40611, t40626, t40632, t40647, t40667, t40673)
}
