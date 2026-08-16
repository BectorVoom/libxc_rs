//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 698/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk698(t2100: f64, t7538: f64, t137: f64, t879: f64, t1089: f64, t1095: f64, t2079: f64, t7458: f64, t7459: f64, t7457: f64, t1967: f64, t2104: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7539 = t7538 * t2100;
    let t7540 = 0.31448092289604152068e-3_f64 * t7539;
    let t7542 = t137 * t879;
    let t7544 = t1089 * t1095 * t7542;
    let t7545 = t2079 * t7544;
    let t7546 = 0.15724046144802076034e-3_f64 * t7545;
    let t7548 = t7458 * t1095 * t7459;
    let t7549 = t7457 * t7548;
    let t7550 = 0.20965394859736101378e-3_f64 * t7549;
    let t7551 = t1967 * t2104;
    (t7540, t7542, t7544, t7546, t7548, t7550, t7551)
}
