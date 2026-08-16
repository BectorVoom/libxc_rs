//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1451/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1451(t18588: f64, t5783: f64, t11237: f64, t1234: f64, t1282: f64, t14816: f64, t18503: f64, t18507: f64, t18518: f64, t18568: f64, t18571: f64, t18580: f64, t18582: f64, t18586: f64, t2448: f64, t2695: f64, t3615: f64, t370: f64, t63: f64, t8245: f64) -> (f64, f64) {
    let t18589 = t5783 * t18588;
    let t18590 = 3.8973666666666666_f64 * t18589;
    let t18591 = t18503 - 1.95872_f64 * t11237 - t18507 + 176.2848_f64 * t63 * t8245 * t2695 * t1234 - 29.3808_f64 * t63 * t3615 * t2448 * t1234 + t18518 - t18571 - 1.46904_f64 * t63 * t370 * t18568 + 11.75232_f64 * t63 * t1282 * t14816 + t18580 - 5.87616_f64 * t18582 + t18586 - t18590;
    (t18590, t18591)
}
