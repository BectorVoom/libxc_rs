//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1494/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1494(t117: f64, t118407: f64, t1459: f64, t1461: f64, t1916: f64, t2187: f64, t21881: f64, t2189: f64, t22544: f64, t22556: f64, t22565: f64, t31117: f64, t31358: f64, t31359: f64, t31362: f64, t31370: f64, t31371: f64, t31374: f64, t31593: f64, t31607: f64, t4292: f64, t572: f64, t5802: f64, t5883: f64, t5920: f64, t6941: f64, t6945: f64, t8273: f64, t8289: f64, t8295: f64, t8296: f64, t8377: f64) -> f64 {
    let t118576 = 12.0_f64 * t572 * t31370 * t4292 + 6.0_f64 * t572 * t5883 * t8273 + 6.0_f64 * t2187 * t22556 + 6.0_f64 * t1916 * t31374 + 12.0_f64 * t1916 * t31359 + 12.0_f64 * t8377 * t5802 + 6.0_f64 * t6941 * t8296 + 12.0_f64 * t1916 * t31362 + 6.0_f64 * t572 * t31358 * t5920 + 6.0_f64 * t572 * t31117 * t5920 + 6.0_f64 * t572 * t8295 * t21881 + 3.0_f64 * t572 * t117 * t118407 + 6.0_f64 * t1459 * t31607 + 6.0_f64 * t8289 * t6945 + 6.0_f64 * t2187 * t22565 + 3.0_f64 * t22544 * t2189 + 3.0_f64 * t31593 * t1461 + 12.0_f64 * t1916 * t31371;
    t118576
}
