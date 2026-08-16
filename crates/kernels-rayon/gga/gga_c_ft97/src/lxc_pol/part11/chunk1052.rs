//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1052/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1052(t1882: f64, t9762: f64, t2371: f64, t2404: f64, t2373: f64, t2405: f64, t446: f64, t713: f64, t9578: f64, t9744: f64, t193: f64, t89: f64, t9692: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41877 = t1882 * t9762;
    let t41879 = t2404 * t2371;
    let t41880 = t2405 * t2373;
    let t41882 = t446 * t41879 * t41880;
    let t41884 = t9578 * t713;
    let t41886 = t446 * t9744 * t41884;
    let t41891 = t89 * t193 * t2371 * t9692 * t713;
    (t41877, t41880, t41882, t41884, t41886, t41891)
}
