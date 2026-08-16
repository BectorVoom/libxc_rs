//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 811/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk811(t420: f64, t8119: f64, t401: f64, t428: f64, t1655: f64, t373: f64, t122: f64, t409: f64, t371: f64, t11174: f64, t17: f64, t110: f64, t1786: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11269 = t420 * t8119;
    let t11335 = t401 * t428;
    let t11351 = t373 * t1655;
    let t11360 = t409 * t122;
    let t11361 = t371 * t11360;
    let t11401 = t11174 * t17;
    let t11468 = t1786 * t110;
    (t11269, t11335, t11351, t11360, t11361, t11401, t11468)
}
