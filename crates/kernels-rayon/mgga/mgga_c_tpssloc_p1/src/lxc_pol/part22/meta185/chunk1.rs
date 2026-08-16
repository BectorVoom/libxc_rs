//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1103/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1103(t1411: f64, t1427: f64, t1434: f64, t5393: f64, t5400: f64, t5403: f64, t5428: f64, t5442: f64, t66: f64, t80: f64) -> f64 {
    let t5445 = -t5393 * t80 / 12.0_f64 - t5400 * t80 / 12.0_f64 - t5403 * t80 / 6.0_f64 - t1411 * t1434 / 6.0_f64 + t5428 * t80 / 24.0_f64 + t1427 * t1434 / 12.0_f64 + t66 * t5442 / 24.0_f64;
    t5445
}
