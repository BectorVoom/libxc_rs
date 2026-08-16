//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 627/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk627(t3588: f64, t3615: f64, t3569: f64, t3573: f64, t3578: f64, t3580: f64, t3583: f64, t3586: f64, t3590: f64, t3597: f64, t360: f64, t3602: f64, t3604: f64, t3607: f64, t3608: f64, t3613: f64, t63: f64) -> (f64, f64) {
    let t3616 = t3615 * t3588;
    let t3619 = -1.46904_f64 * t3569 + 2.20356_f64 * t3573 + t3578 + t3580 - 2.93808_f64 * t3583 - 3.0_f64 / 2.0_f64 * t3586 - 6.0_f64 * t360 * t3590 - 8.81424_f64 * t3597 - t3602 - t3604 - t3607 - 1.46904_f64 * t63 * t3608 - t3613 - 29.3808_f64 * t63 * t3616;
    (t3616, t3619)
}
