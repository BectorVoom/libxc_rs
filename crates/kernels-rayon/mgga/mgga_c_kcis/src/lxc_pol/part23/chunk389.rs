//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 389/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk389(t209: f64, t2410: f64, t698: f64, t2394: f64, t2399: f64, t2406: f64, t63: f64, t696: f64, t702: f64, t75: f64, t706: f64, t124: f64, t691: f64) -> (f64, f64, f64, f64) {
    let t2412 = t209 * t698 * t2410;
    let t2415 = 35.0_f64 / 432.0_f64 * t63 * t2394 * t75 + 7.0_f64 / 144.0_f64 * t2399 * t702 + t696 * t2406 / 48.0_f64 - t696 * t2412 / 96.0_f64;
    let t2416 = t2415 * t706;
    let t2421 = t691 * t124;
    (t2412, t2415, t2416, t2421)
}
