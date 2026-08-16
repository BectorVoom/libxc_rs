//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 667/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk667(t9162: f64, t9257: f64, t605: f64, t144: f64, t167: f64, t574: f64, t9007: f64, t2075: f64, t616: f64, t576: f64, t8232: f64, t611: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9258 = t9162 + t9257;
    let t9259 = t605 * t9258;
    let t9260 = t144 * t9259;
    let t9264 = t574 * t167 * t9007;
    let t9268 = t574 * t616 * t2075;
    let t9270 = t8232 * t576;
    let t9272 = t8232 * t611;
    (t9258, t9259, t9260, t9264, t9268, t9270, t9272)
}
