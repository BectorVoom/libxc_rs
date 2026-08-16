//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2180/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2180(t97233: f64, t97268: f64, t97309: f64, t97349: f64, t97376: f64, t97392: f64, t97433: f64, t97465: f64, t19661: f64, t1992: f64, t22897: f64, t19736: f64) -> (f64, f64, f64) {
    let t97468 = t97233 + t97268 + t97309 + t97349 + t97376 + t97392 + t97433 + t97465;
    let t97488 = t1992 * t22897 * t19661;
    let t97491 = t1992 * t22897 * t19736;
    (t97468, t97488, t97491)
}
