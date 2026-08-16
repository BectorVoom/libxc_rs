//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1436/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1436(t19270: f64, t193: f64, t336: f64, t3640: f64, t4700: f64, t6270: f64, t78310: f64, t78312: f64, t78314: f64, t78318: f64, t78320: f64, t78321: f64, t78327: f64, t78329: f64, t78331: f64, t78333: f64, t78335: f64, t78338: f64) -> f64 {
    let t78342 = -3.0_f64 * t193 * t336 * t3640 * t78321 + 12.0_f64 * t19270 * t4700 * t6270 + t78310 - t78312 - t78314 - t78318 - t78320 + t78327 + t78329 + t78331 + t78333 + t78335 + t78338;
    t78342
}
