//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 462/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk462(t3413: f64, t515: f64, t1053: f64, t604: f64, t379: f64, t2210: f64, t558: f64, t920: f64, t2222: f64, t2221: f64, t609: f64, t2211: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3414 = t515 * t3413;
    let t3419 = t604 * t1053;
    let t3420 = t3419 * t379;
    let t3421 = t2210 * t3420;
    let t3424 = t920 * t558;
    let t3425 = t2222 * t3424;
    let t3426 = t2221 * t3425;
    let t3429 = t920 * t609;
    let t3430 = t2211 * t3429;
    (t3414, t3420, t3421, t3425, t3426, t3430)
}
