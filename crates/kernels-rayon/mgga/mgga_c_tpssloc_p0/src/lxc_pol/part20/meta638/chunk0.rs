//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2345/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2345(t136: f64, t2826: f64, t47726: f64, t13560: f64, t699: f64, t47759: f64, t908: f64, t47689: f64, t2403: f64, t4392: f64, t13646: f64, t47734: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t48085 = t136 * t2826 * t47726;
    let t48087 = t699 * t13560;
    let t48090 = t136 * t908 * t47759;
    let t48092 = t136 * t2826 * t47689;
    let t48096 = t2403 * t4392;
    let t48097 = 5.0_f64 / 9.0_f64 * t48096;
    let t48098 = t699 * t13646;
    let t48101 = t136 * t908 * t47734;
    (t48085, t48087, t48090, t48092, t48096, t48097, t48098, t48101)
}
