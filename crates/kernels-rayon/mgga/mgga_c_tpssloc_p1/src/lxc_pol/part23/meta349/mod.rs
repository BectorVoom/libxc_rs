//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta349 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1143;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1144;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta349(t40419: f64, t535: f64, t9538: f64, t241: f64, t6597: f64, t248: f64, t555: f64, t557: f64, t40041: f64, t562: f64, t12019: f64, t566: f64, t68: f64, t3700: f64, t195: f64, t632: f64, t197: f64, t636: f64, t39264: f64, t761: f64, t39259: f64, t39358: f64, t756: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40422, t40445, t40449, t40541, t40590) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1143(t40419, t535, t9538, t241, t6597, t248, t555, t557, t40041, t562, t12019, t566);
        let (t40591, t40611, t40632, t40647, t40679, t40685, t40708) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1144(t40590, t68, t3700, t195, t632, t197, t636, t39264, t761, t39259, t39358, t756);
    (t40422, t40445, t40449, t40541, t40591, t40611, t40632, t40647, t40679, t40685, t40708)
}
