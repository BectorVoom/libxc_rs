//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1065/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1065(t26592: f64, t26631: f64, t165: f64, t2165: f64, t2789: f64, t26521: f64, t26523: f64, t26525: f64, t26528: f64, t26531: f64, t26534: f64, t26536: f64, t26538: f64, t26540: f64, t26542: f64, t26545: f64, t26548: f64, t26551: f64, t26554: f64) -> (f64, f64, f64, f64) {
    let t26632 = t26592 + t26631;
    let t26633 = t26632 * t165;
    let t26634 = t2165 * t2789;
    let t26651 = t26521 / 8.0_f64 - t26523 / 4.0_f64 - t26525 / 2.0_f64 + t26528 / 4.0_f64 + t26531 / 2.0_f64 - t26534 / 8.0_f64 + 3.0_f64 / 4.0_f64 * t26536 - t26538 / 32.0_f64 + t26540 / 16.0_f64 + t26542 / 4.0_f64 - t26545 / 16.0_f64 - t26548 / 4.0_f64 + t26551 / 32.0_f64 - 5.0_f64 / 8.0_f64 * t26554;
    (t26632, t26633, t26634, t26651)
}
