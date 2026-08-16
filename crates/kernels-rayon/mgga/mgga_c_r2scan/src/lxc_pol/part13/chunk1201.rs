//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1201/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1201(t11011: f64, t11479: f64, t3262: f64, t11502: f64, t37513: f64, t10940: f64, t11540: f64, t38299: f64, t897: f64, t10680: f64, t38301: f64, t10918: f64, t3275: f64, t7040: f64) -> (f64, f64, f64, f64, f64) {
    let t40404 = 3.0_f64 / 2.0_f64 * t3262 * t11479 * t11011;
    let t40406 = 3.0_f64 / 4.0_f64 * t37513 * t11502;
    let t40408 = t10940 * t11540 / 4.0_f64;
    let t40409 = t38299 * t897;
    let t40411 = t10680 * t40409 * t38301;
    let t40415 = t3275 * t10918 * t7040 / 2.0_f64;
    (t40404, t40406, t40408, t40411, t40415)
}
