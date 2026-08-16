//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 203/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk203(t135: f64, t144: f64, t469: f64, t494: f64, t498: f64, t503: f64, t514: f64, t547: f64, t549: f64, t554: f64, t559: f64, t560: f64, t568: f64, t637: f64, t639: f64) -> f64 {
    let t642 = t135 * t144 * t637 * t639 + 3.0_f64 * t135 * t560 * t568 + t469 + t494 + t498 - t503 + t514 + t547 + t549 - t554 - t559;
    t642
}
