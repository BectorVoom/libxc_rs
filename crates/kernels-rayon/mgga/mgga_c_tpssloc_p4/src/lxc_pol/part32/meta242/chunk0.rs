//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1097/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1097(t3887: f64, t6439: f64, t3897: f64, t6388: f64, t1825: f64, t5348: f64, t1380: f64, t6415: f64, t6420: f64, t553: f64, t6434: f64, t1336: f64, t1814: f64, t1838: f64, t1840: f64, t5234: f64, t544: f64, t564: f64, t6378: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6440 = t3887 * t6439;
    let t6448 = t3897 * t6388;
    let t6451 = t5348 * t1825;
    let t6454 = t1380 * t6415;
    let t6456 = t1380 * t6420;
    let t6458 = t553 * t6434;
    let t6460 = 2.0_f64 * t1336 * t6448 - 2.0_f64 * t1336 * t6451 - t1336 * t6454 - t1336 * t6456 + 2.0_f64 * t1814 * t1840 - 2.0_f64 * t1838 * t5234 + t544 * t6458 + t564 * t6378;
    (t6440, t6448, t6451, t6454, t6456, t6458, t6460)
}
