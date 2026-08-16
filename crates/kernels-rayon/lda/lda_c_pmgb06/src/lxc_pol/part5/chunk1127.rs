//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1127/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1127(t13244: f64, t20515: f64, t20516: f64, t20517: f64, t20518: f64, t20519: f64, t20520: f64, t20521: f64, t20523: f64, t20525: f64, t20529: f64, t16563: f64, t1893: f64, t5077: f64) -> (f64, f64) {
    let t20530 = t20515 + t20516 + t20517 + t20518 + t20519 - t20520 + t20521 - t20523 - t20525 - t20529 - t13244;
    let t20533 = 2.0_f64 / 15.0_f64 * t5077 * t16563 * t1893;
    (t20530, t20533)
}
