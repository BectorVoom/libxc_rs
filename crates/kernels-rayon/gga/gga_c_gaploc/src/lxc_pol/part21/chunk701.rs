//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 701/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk701(t6341: f64, t6422: f64, t6482: f64, t6549: f64, t2353: f64, t501: f64, t1381: f64, t892: f64, t1383: f64, t921: f64, t2497: f64, t605: f64) -> (f64, f64, f64, f64, f64) {
    let t6551 = t6341 + t6422 + t6482 + t6549;
    let t6553 = t2353 * t501;
    let t6556 = t892 * t1381;
    let t6565 = t921 * t1383;
    let t6568 = t2497 * t605;
    (t6551, t6553, t6556, t6565, t6568)
}
