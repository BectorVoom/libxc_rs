//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 632/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk632(t1233: f64, t1694: f64, t187: f64, t5261: f64, t6362: f64, t6364: f64, t6368: f64, t6392: f64, t6395: f64, t6401: f64, t6408: f64, t6425: f64, t6429: f64, t6823: f64) -> f64 {
    let t6835 = -t6362 + t6364 - t6368 + t6392 + t6395 + t187 * t6823 + 0.19751789702565206229e-1_f64 * t187 * t6401 - 0.11696446794910408142e1_f64 * t5261 * t1694 + 0.11696446794910408142e1_f64 * t1233 * t6408 - 0.58482233974552040708e0_f64 * t1233 * t6425 - 0.17315755899375863299e2_f64 * t1233 * t6429;
    t6835
}
