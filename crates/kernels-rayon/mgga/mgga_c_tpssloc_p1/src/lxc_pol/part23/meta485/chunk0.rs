//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1484/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1484(t1410: f64, t1434: f64, t1864: f64, t19322: f64, t20207: f64, t20217: f64, t20222: f64, t20227: f64, t20264: f64, t20265: f64, t33: f64, t5398: f64, t5399: f64, t5400: f64, t5427: f64, t5442: f64, t65: f64, t7445: f64, t75361: f64, t75847: f64, t79692: f64, t80: f64) -> f64 {
    let t79707 = -t5399 * t5427 * t80 / 2.0_f64 - t20222 * t1434 - t5400 * t5442 / 2.0_f64 - t1410 * t20264 * t80 / 3.0_f64 - t20227 * t1434 + t33 * t79692 * t80 / 24.0_f64 + t20265 * t1434 / 6.0_f64 - t75847 * t65 * t80 / 4.0_f64 - t75361 * t20207 - t19322 * t7445 * t5398 - t19322 * t1864 * t20217 / 3.0_f64;
    t79707
}
