//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1144/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1144(t40590: f64, t68: f64, t3700: f64, t195: f64, t632: f64, t197: f64, t636: f64, t39264: f64, t761: f64, t39259: f64, t39358: f64, t756: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40591 = t68 * t40590;
    let t40610 = t3700 * t3700;
    let t40611 = 1.0_f64 / t40610;
    let t40632 = 1.0_f64 / t195 / t632;
    let t40647 = 1.0_f64 / t197 / t636;
    let t40679 = 0.61524113149298439947e4_f64 * t761 * t39264;
    let t40685 = 0.69263436422725855036e2_f64 * t761 * t39259;
    let t40708 = 0.18989649058080861537e-2_f64 * t756 * t39358;
    (t40591, t40611, t40632, t40647, t40679, t40685, t40708)
}
