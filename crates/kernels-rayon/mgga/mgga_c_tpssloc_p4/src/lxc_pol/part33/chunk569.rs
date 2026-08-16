//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 569/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk569(t2291: f64, t5392: f64, t5398: f64, t634: f64, t2298: f64, t638: f64, t72: f64, t1411: f64, t1427: f64, t1434: f64, t5393: f64, t5400: f64, t5403: f64, t5428: f64, t66: f64, t80: f64) -> (f64, f64, f64) {
    let t5433 = t2291 * t5392;
    let t5435 = t634 * t5398;
    let t5437 = t2298 * t5392;
    let t5439 = t638 * t5398;
    let t5441 = 28.0_f64 / 9.0_f64 * t5433 - 4.0_f64 / 3.0_f64 * t5435 + 28.0_f64 / 9.0_f64 * t5437 + 4.0_f64 / 3.0_f64 * t5439;
    let t5442 = t72 * t5441;
    let t5445 = -t5393 * t80 / 12.0_f64 - t5400 * t80 / 12.0_f64 - t5403 * t80 / 6.0_f64 - t1411 * t1434 / 6.0_f64 + t5428 * t80 / 24.0_f64 + t1427 * t1434 / 12.0_f64 + t66 * t5442 / 24.0_f64;
    (t5441, t5442, t5445)
}
