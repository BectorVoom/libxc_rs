//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1480/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1480(t1256: f64, t1763: f64, t193: f64, t336: f64, t43706: f64, t4700: f64, t71101: f64, t78344: f64, t78348: f64, t78355: f64, t78357: f64, t78359: f64, t78361: f64, t78364: f64, t78367: f64, t78370: f64, t78373: f64, t78646: f64, t79005: f64, t79533: f64) -> f64 {
    let t79538 = -t78344 - 4.0_f64 * t4700 * t71101 * t1763 - 6.0_f64 * t193 * t336 * t78348 * t43706 + t78355 - t78357 - t78359 + t78361 - t78364 - t78367 + t78370 + t78373 + t193 * t336 * (t78646 + t79533) * t1256 + t79005;
    t79538
}
