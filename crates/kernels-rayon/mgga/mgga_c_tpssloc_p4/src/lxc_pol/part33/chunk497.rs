//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 497/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk497(t374: f64, t376: f64, t677: f64, t370: f64, t121: f64, t1013: f64, t361: f64) -> (f64, f64, f64, f64) {
    let t3082 = t374 * t677 * t376;
    let t3084 = t370 * t3082 / 13824.0_f64;
    let t3101 = t121 * t376;
    let t3127 = 1.0_f64 / t1013 / t361;
    (t3082, t3084, t3101, t3127)
}
