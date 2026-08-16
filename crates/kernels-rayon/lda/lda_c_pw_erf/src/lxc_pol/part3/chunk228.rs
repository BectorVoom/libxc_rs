//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 228/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk228(t226: f64, t611: f64, t230: f64, t231: f64, t498: f64, t513: f64, t517: f64, t527: f64, t546: f64, t553: f64, t567: f64, t570: f64, t579: f64, t597: f64, t598: f64, t606: f64) -> (f64, f64, f64) {
    let t613 = 4.0_f64 / 3.0_f64 * t226 * t611;
    let t615 = 4.0_f64 / 3.0_f64 * t226 * t230;
    let t616 = t498 + t513 + t517 + t527 - t546 + t553 + t567 + t570 + t579 - t597 + 4.0_f64 / 3.0_f64 * t598 * t231 + t606 + t613 + t615;
    (t613, t615, t616)
}
