//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2050/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2050(t39264: f64, t761: f64, t2531: f64, t9905: f64, t39259: f64, t39358: f64, t756: f64, t187: f64, t268: f64, t39322: f64, t39347: f64, t39336: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40679 = 0.61524113149298439947e4_f64 * t761 * t39264;
    let t40682 = t2531 * t9905;
    let t40685 = 0.69263436422725855036e2_f64 * t761 * t39259;
    let t40708 = 0.18989649058080861537e-2_f64 * t756 * t39358;
    let t40712 = t187 * t268;
    let t40714 = 0.1301229756036208781e0_f64 * t40712 * t39322;
    let t40716 = 0.19263893255070628431e1_f64 * t40712 * t39347;
    let t40721 = 0.21053605041484726346e2_f64 * t761 * t39336;
    (t40679, t40682, t40685, t40708, t40714, t40716, t40721)
}
