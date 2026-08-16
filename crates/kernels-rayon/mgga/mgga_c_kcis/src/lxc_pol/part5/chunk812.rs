//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 812/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk812(t6423: f64, t962: f64, t971: f64, t3031: f64, t6406: f64, t3034: f64, t1694: f64, t45: f64, t4735: f64, t6362: f64, t6364: f64, t6368: f64, t6392: f64, t6395: f64, t6401: f64, t6408: f64, t960: f64) -> (f64, f64, f64, f64) {
    let t6425 = t962 * t6423 * t971;
    let t6428 = t3031 * t6406;
    let t6429 = t6428 * t3034;
    let t6432 = -t6362 + t6364 - t6368 + t6392 + t6395 + 0.19751789702565206229e-1_f64 * t45 * t6401 - 0.11696446794910408142e1_f64 * t4735 * t1694 + 0.11696446794910408142e1_f64 * t960 * t6408 - 0.58482233974552040708e0_f64 * t960 * t6425 - 0.17315755899375863299e2_f64 * t960 * t6429;
    (t6425, t6428, t6429, t6432)
}
