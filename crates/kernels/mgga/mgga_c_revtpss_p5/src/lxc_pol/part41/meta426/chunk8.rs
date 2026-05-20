//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1494/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1494<F: Float>(t117: F, t118407: F, t1459: F, t1461: F, t1916: F, t2187: F, t21881: F, t2189: F, t22544: F, t22556: F, t22565: F, t31117: F, t31358: F, t31359: F, t31362: F, t31370: F, t31371: F, t31374: F, t31593: F, t31607: F, t4292: F, t572: F, t5802: F, t5883: F, t5920: F, t6941: F, t6945: F, t8273: F, t8289: F, t8295: F, t8296: F, t8377: F) -> F {
    let t118576 = F::new(12.0) * t572 * t31370 * t4292 + F::new(6.0) * t572 * t5883 * t8273 + F::new(6.0) * t2187 * t22556 + F::new(6.0) * t1916 * t31374 + F::new(12.0) * t1916 * t31359 + F::new(12.0) * t8377 * t5802 + F::new(6.0) * t6941 * t8296 + F::new(12.0) * t1916 * t31362 + F::new(6.0) * t572 * t31358 * t5920 + F::new(6.0) * t572 * t31117 * t5920 + F::new(6.0) * t572 * t8295 * t21881 + F::new(3.0) * t572 * t117 * t118407 + F::new(6.0) * t1459 * t31607 + F::new(6.0) * t8289 * t6945 + F::new(6.0) * t2187 * t22565 + F::new(3.0) * t22544 * t2189 + F::new(3.0) * t31593 * t1461 + F::new(12.0) * t1916 * t31371;
    t118576
}
