//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 775/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk775(t1080: f64, t5280: f64, t2991: f64, t493: f64, t1420: f64, t1894: f64, t1893: f64, t2948: f64, t439: f64, t1629: f64, t809: f64, t1385: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5281 = t5280 * t1080;
    let t5282 = t2991 * t5281;
    let t5284 = t493 * t5282 / 27.0_f64;
    let t5286 = 2.0_f64 / 45.0_f64 * t1420 * t1894;
    let t5287 = t2948 * t1893;
    let t5289 = 2.0_f64 / 45.0_f64 * t439 * t5287;
    let t5290 = t809 * t1629;
    let t5291 = t1385 * t5290;
    (t5281, t5282, t5284, t5286, t5287, t5289, t5290, t5291)
}
