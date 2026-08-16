//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 686/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk686(t11360: f64, t371: f64, t408: f64, t929: f64, t11174: f64, t17: f64, t355: f64, t3001: f64, t89: f64, t3014: f64, t376: f64, t3196: f64, t8392: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11361 = t371 * t11360;
    let t11375 = t408 * t929;
    let t11401 = t11174 * t17;
    let t11402 = t11401 * t355;
    let t11404 = t89 * t11402 * t3001;
    let t11416 = t89 * t376 * t3014;
    let t11417 = 2.0_f64 / 9.0_f64 * t11416;
    let t11430 = 4.0_f64 / 81.0_f64 * t8392 * t3196;
    (t11361, t11375, t11401, t11402, t11404, t11416, t11417, t11430)
}
