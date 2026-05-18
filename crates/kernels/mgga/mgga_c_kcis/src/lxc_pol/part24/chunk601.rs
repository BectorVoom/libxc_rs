//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 601/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk601<F: Float>(t6423: F, t962: F, t971: F, t3031: F, t6406: F, t3034: F, t1694: F, t45: F, t4735: F, t6362: F, t6364: F, t6368: F, t6392: F, t6395: F, t6401: F, t6408: F, t960: F) -> (F, F, F, F) {
    let t6425 = t962 * t6423 * t971;
    let t6428 = t3031 * t6406;
    let t6429 = t6428 * t3034;
    let t6432 = -t6362 + t6364 - t6368 + t6392 + t6395 + F::new(0.19751789702565206229e-1) * t45 * t6401 - F::new(0.11696446794910408142e1) * t4735 * t1694 + F::new(0.11696446794910408142e1) * t960 * t6408 - F::new(0.58482233974552040708e0) * t960 * t6425 - F::new(0.17315755899375863299e2) * t960 * t6429;
    (t6425, t6428, t6429, t6432)
}
