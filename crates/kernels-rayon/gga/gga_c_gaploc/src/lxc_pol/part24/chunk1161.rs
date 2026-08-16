//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1161/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1161(t10290: f64, t10304: f64, t10298: f64, t4349: f64, t605: f64, t1651: f64, t3366: f64, t27214: f64, t6565: f64, t6568: f64, t8045: f64, t1382: f64, t3418: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31454 = 4.0_f64 * t10290;
    let t31455 = 2.0_f64 * t10304;
    let t31458 = 12.0_f64 * t4349 * t10298 * t605;
    let t31461 = 6.0_f64 * t4349 * t3366 * t1651;
    let t31463 = 6.0_f64 * t27214 * t6565;
    let t31465 = 4.0_f64 * t8045 * t6568;
    let t31468 = 2.0_f64 * t1382 * t3418 * t1651;
    (t31454, t31455, t31458, t31461, t31463, t31465, t31468)
}
