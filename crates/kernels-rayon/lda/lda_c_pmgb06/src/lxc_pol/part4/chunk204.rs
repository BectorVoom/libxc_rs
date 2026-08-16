//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 204/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk204(t543: f64, t545: f64, t184: f64, t187: f64, t188: f64, t434: f64, t438: f64, t448: f64, t462: f64, t481: f64, t488: f64, t492: f64, t502: f64, t515: f64, t533: f64, t534: f64, t542: f64) -> (f64, f64, f64) {
    let t547 = 0.10821041362364843_f64 * t543 * t545;
    let t549 = 4.0_f64 / 3.0_f64 * t184 * t187;
    let t550 = t434 + t438 + t448 + t462 - t481 + t488 + t492 + t502 + t515 - t533 + 4.0_f64 / 3.0_f64 * t534 * t188 + t542 + t547 + t549;
    (t547, t549, t550)
}
