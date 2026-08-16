//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta549 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2048;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2049;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta549(t2229: f64, t59: f64, t60: f64, t535: f64, t9538: f64, t241: f64, t6597: f64, t248: f64, t555: f64, t557: f64, t12248: f64, t1372: f64, t12019: f64, t566: f64, t68: f64, t3700: f64, t195: f64, t632: f64, t197: f64, t636: f64, t2531: f64, t9892: f64, t718: f64, t9862: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40419, t40422, t40445, t40449, t40492) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2048(t2229, t59, t60, t535, t9538, t241, t6597, t248, t555, t557, t12248, t1372);
        let (t40591, t40611, t40632, t40647, t40667, t40673) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2049(t12019, t566, t68, t3700, t195, t632, t197, t636, t2531, t9892, t718, t9862);
    (t40419, t40422, t40445, t40449, t40492, t40591, t40611, t40632, t40647, t40667, t40673)
}
