//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1211/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1211(t10615: f64, t11518: f64, t3262: f64, t10918: f64, t11475: f64, t11515: f64, t11523: f64, t11550: f64, t11514: f64, t1551: f64, t3579: f64, t113: f64, t36985: f64, t97: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40536 = 15.0_f64 / 8.0_f64 * t3262 * t10615 * t11518;
    let t40539 = 3.0_f64 / 2.0_f64 * t3262 * t10918 * t11475;
    let t40541 = t11523 * t11515 / 2.0_f64;
    let t40544 = 3.0_f64 / 2.0_f64 * t3262 * t10918 * t11550;
    let t40547 = t3579 * t1551 * t11514 / 4.0_f64;
    let t40549 = t97 * t36985 * t113;
    (t40536, t40539, t40541, t40544, t40547, t40549)
}
