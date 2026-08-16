//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2654/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2654(t12633: f64, t12636: f64, t12648: f64, t1426: f64, t1434: f64, t19331: f64, t19334: f64, t19335: f64, t19338: f64, t2252: f64, t2255: f64, t2283: f64, t2304: f64, t31: f64, t3976: f64, t4018: f64, t5399: f64, t5400: f64, t5428: f64, t5442: f64, t55677: f64, t628: f64, t642: f64, t65: f64, t80: f64) -> f64 {
    let t55709 = -t19331 * t642 / 6.0_f64 - t31 * t55677 * t65 * t80 / 12.0_f64 - t19334 * t628 * t80 / 6.0_f64 - t19335 * t642 / 6.0_f64 - t5399 * t2283 * t80 / 12.0_f64 - t19338 * t642 / 6.0_f64 - t5400 * t2304 / 12.0_f64 - t12648 * t1426 * t80 / 6.0_f64 - t12633 * t1434 / 6.0_f64 - t12636 * t1434 / 3.0_f64 - t3976 * t4018 / 3.0_f64 - t2252 * t5442 / 12.0_f64 - t2255 * t5442 / 6.0_f64 + t5428 * t2304 / 24.0_f64;
    t55709
}
