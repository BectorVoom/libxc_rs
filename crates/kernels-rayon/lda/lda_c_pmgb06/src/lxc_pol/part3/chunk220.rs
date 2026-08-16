//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 220/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk220(t223: f64, t607: f64, t213: f64, t224: f64, t434: f64, t438: f64, t448: f64, t462: f64, t481: f64, t488: f64, t492: f64, t502: f64, t515: f64, t533: f64, t574: f64, t583: f64, t590: f64, t593: f64, t604: f64) -> (f64, f64) {
    let t609 = 2.0_f64 / 45.0_f64 * t223 * t607;
    let t610 = t434 + t438 + t448 + t462 - t481 + t488 + t492 + t502 + t515 - t533 + t574 * t213 / 3.0_f64 + t583 + t590 + t593 - t604 * t224 / 15.0_f64 - t609;
    (t609, t610)
}
