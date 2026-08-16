//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 669/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk669(t19278: f64, t19298: f64, t19301: f64, t19304: f64, t5: f64, t5429: f64, t4417: f64, t7712: f64, t7720: f64, t1528: f64, t4431: f64, t4495: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19852 = 2.0_f64 / 9.0_f64 * t19278;
    let t19857 = t19298 / 9.0_f64;
    let t19858 = 2.0_f64 / 9.0_f64 * t19301;
    let t19859 = 2.0_f64 / 27.0_f64 * t19304;
    let t19920 = t5 * t5429;
    let t19950 = t7712 * t4417;
    let t19957 = t7720 * t4417;
    let t19961 = t1528 * t4431;
    let t19965 = t72 * t4495;
    (t19852, t19857, t19858, t19859, t19920, t19950, t19957, t19961, t19965)
}
