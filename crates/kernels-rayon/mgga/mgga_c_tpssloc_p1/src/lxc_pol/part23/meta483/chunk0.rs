//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1463/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1463(t300: f64, t78874: f64, t78914: f64, t78944: f64, t79002: f64, t78335: f64, t78338: f64, t78344: f64, t78355: f64, t78357: f64, t78359: f64, t78361: f64, t78364: f64, t78367: f64, t78370: f64, t78373: f64) -> (f64, f64) {
    let t79005 = t300 * (t78874 + t78914 + t78944 + t79002);
    let t79006 = t78335 + t78338 - t78344 + t78355 - t78357 - t78359 + t78361 - t78364 - t78367 + t78370 + t78373 + t79005;
    (t79005, t79006)
}
