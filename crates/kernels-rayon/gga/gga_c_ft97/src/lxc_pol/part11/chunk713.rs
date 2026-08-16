//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 713/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk713(t9707: f64, t9708: f64, t27: f64, t89: f64, t2371: f64, t2459: f64, t713: f64, t193: f64, t9567: f64, t241: f64, t9570: f64, t9571: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9709 = t9707 * t9708;
    let t9711 = t89 * t27 * t9709;
    let t9713 = t2371 * t713 * t2459;
    let t9715 = t89 * t193 * t9713;
    let t9716 = t27 * t9567;
    let t9717 = t241 * t9570;
    let t9718 = t9717 * t9571;
    (t9709, t9711, t9713, t9715, t9716, t9717, t9718)
}
