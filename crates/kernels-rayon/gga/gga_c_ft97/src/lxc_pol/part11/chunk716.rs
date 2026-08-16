//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 716/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk716(t670: f64, t89: f64, t9733: f64, t2348: f64, t9571: f64, t666: f64, t724: f64, t9592: f64, t446: f64, t2404: f64, t675: f64, t2405: f64, t713: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9735 = t89 * t9733 * t670;
    let t9737 = t2348 * t9571;
    let t9739 = t89 * t666 * t9737;
    let t9741 = t724 * t9592;
    let t9742 = t446 * t9741;
    let t9744 = t2404 * t675;
    let t9745 = t2405 * t713;
    (t9735, t9737, t9739, t9741, t9742, t9744, t9745)
}
