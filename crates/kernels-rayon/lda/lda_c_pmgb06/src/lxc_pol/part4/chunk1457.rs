//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1457/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1457(t11400: f64, t11402: f64, t8300: f64, t8306: f64, t8328: f64, t11395: f64, t11398: f64, t11405: f64, t8313: f64, t8324: f64, t8339: f64, t8346: f64, t8353: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18693 = 2.5982444444444446_f64 * t11400;
    let t18694 = 2.5982444444444446_f64 * t11402;
    let t18696 = 0.6495611111111111_f64 * t8300;
    let t18697 = 2.5982444444444446_f64 * t8306;
    let t18700 = 0.3247805555555556_f64 * t8328;
    let t18702 = -3.91744_f64 * t11395 + 3.91744_f64 * t11398 - t18693 - t18694 - 8.0_f64 / 9.0_f64 * t11405 - t18696 - t18697 - 2.0_f64 / 9.0_f64 * t8313 - 0.48968_f64 * t8324 + t18700 - t8339 + t8346 + 4.570346666666667_f64 * t8353;
    (t18693, t18694, t18696, t18697, t18700, t18702)
}
