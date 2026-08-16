//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2405/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2405(t39264: f64, t761: f64, t2663: f64, t9901: f64, t2531: f64, t9905: f64, t39259: f64, t2250: f64, t2517: f64, t707: f64, t39358: f64, t756: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40679 = 0.61524113149298439947e4_f64 * t761 * t39264;
    let t40680 = t9901 * t2663;
    let t40682 = t2531 * t9905;
    let t40685 = 0.69263436422725855036e2_f64 * t761 * t39259;
    let t40687 = t707 * t2517 * t2250;
    let t40708 = 0.18989649058080861537e-2_f64 * t756 * t39358;
    (t40679, t40680, t40682, t40685, t40687, t40708)
}
