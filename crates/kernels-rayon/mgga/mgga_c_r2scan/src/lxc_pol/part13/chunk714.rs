//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 714/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk714(t224: f64, t718: f64, t1981: f64, t1719: f64, t1821: f64, t695: f64, t1945: f64, t4741: f64, t5246: f64, t5416: f64, t5418: f64, t5422: f64, t5424: f64, t5426: f64) -> (f64, f64, f64, f64, f64) {
    let t5564 = t718 * t224;
    let t5567 = t1981 * t224;
    let t5568 = t1821 * t1719;
    let t5569 = t5568 * t695;
    let t5572 = t1945 * t224;
    let t5582 = 0.126595e2_f64 * t5246 - 0.50638000000000000001e1_f64 * t5416 + 0.78770222222222222223e1_f64 * t5418 - 0.81910000000000000002e0_f64 * t5422 + 0.54606666666666666667e0_f64 * t5424 - 0.63707777777777777777e0_f64 * t5426 - 0.25559851851851851851e0_f64 * t4741;
    (t5564, t5567, t5569, t5572, t5582)
}
