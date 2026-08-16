//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 324/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk324(t3578: f64, t609: f64, t144: f64, t1053: f64, t2142: f64, t2140: f64, t2165: f64, t2167: f64, t28: f64, t3480: f64, t3485: f64, t3489: f64, t3541: f64, t3545: f64, t3548: f64, t3551: f64, t3567: f64, t3571: f64, t3575: f64, t446: f64, t89: f64) -> f64 {
    let t3579 = t3578 * t609;
    let t3580 = t144 * t3579;
    let t3583 = t2142 * t1053;
    let t3584 = t144 * t3583;
    let t3587 = t2165 / 9.0_f64 + t2167 / 9.0_f64 - t2140 / 9.0_f64 + t446 * t3480 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t3485 - t3489 / 9.0_f64 + t89 * t28 * t3541 / 3.0_f64 + t3545 / 9.0_f64 - t446 * t3548 / 3.0_f64 + t3551 / 9.0_f64 - t446 * t3567 / 3.0_f64 - t446 * t3571 / 3.0_f64 - t446 * t3575 / 3.0_f64 - t446 * t3580 / 3.0_f64 - t446 * t3584 / 3.0_f64;
    t3587
}
