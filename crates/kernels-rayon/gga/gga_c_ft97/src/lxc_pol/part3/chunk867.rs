//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 867/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk867(t17454: f64, t17459: f64, t17472: f64, t17484: f64, t143: f64, t160: f64, t12703: f64, t16666: f64, t3455: f64, t925: f64, t9144: f64, t2142: f64, t4805: f64) -> (f64, f64, f64, f64, f64) {
    let t17486 = t17454 + t17459 + t17472 + t17484;
    let t17488 = t143 * t17486 * t160;
    let t17493 = t12703 * t16666;
    let t17496 = t925 * t3455;
    let t17497 = t9144 * t17496;
    let t17500 = t2142 * t4805;
    (t17486, t17488, t17493, t17497, t17500)
}
