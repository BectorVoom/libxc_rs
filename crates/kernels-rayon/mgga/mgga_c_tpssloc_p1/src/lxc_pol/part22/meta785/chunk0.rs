//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2703/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2703(t1410: f64, t1434: f64, t19335: f64, t19338: f64, t19343: f64, t19346: f64, t19349: f64, t19404: f64, t20227: f64, t3961: f64, t3967: f64, t4018: f64, t5400: f64, t5403: f64, t5427: f64, t642: f64, t80: f64) -> f64 {
    let t75419 = -t19335 * t1434 / 4.0_f64 - t19338 * t1434 / 4.0_f64 - t5400 * t4018 / 4.0_f64 - t3961 * t5427 * t80 / 4.0_f64 - t3967 * t5427 * t80 / 4.0_f64 - t1410 * t19404 * t80 / 4.0_f64 - t20227 * t642 / 4.0_f64 - t19343 * t1434 / 2.0_f64 - t19346 * t1434 / 2.0_f64 - t19349 * t1434 / 2.0_f64 - t5403 * t4018 / 2.0_f64;
    t75419
}
