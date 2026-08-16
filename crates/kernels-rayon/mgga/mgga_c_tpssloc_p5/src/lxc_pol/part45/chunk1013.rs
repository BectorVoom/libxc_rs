//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1013/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1013(t1992: f64, t550: f64, t6976: f64, t84441: f64, t22704: f64, t22705: f64, t31627: f64, t1351: f64, t7191: f64, t31632: f64, t6883: f64, t22724: f64, t31623: f64) -> (f64, f64, f64, f64, f64) {
    let t115420 = t1992 * t6976 * t84441 * t550;
    let t115423 = t22704 * t22705 * t31627;
    let t115428 = t1992 * t6976 * t7191 * t1351 * t550;
    let t115430 = t6883 * t31632;
    let t115432 = t22724 * t31623;
    (t115420, t115423, t115428, t115430, t115432)
}
