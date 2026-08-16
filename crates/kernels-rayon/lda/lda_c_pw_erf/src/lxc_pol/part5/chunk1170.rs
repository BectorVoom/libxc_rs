//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1170/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1170(t6303: f64, t822: f64, t17058: f64, t17060: f64, t17063: f64, t11057: f64, t11060: f64, t11063: f64, t11065: f64, t11069: f64, t11073: f64, t11074: f64, t11079: f64, t11081: f64, t11088: f64) -> (f64, f64, f64, f64, f64) {
    let t21362 = t822 * t6303;
    let t21363 = 8.0_f64 / 15.0_f64 * t21362;
    let t21364 = 8.0_f64 / 15.0_f64 * t17058;
    let t21365 = 8.0_f64 / 15.0_f64 * t17060;
    let t21366 = 8.0_f64 / 15.0_f64 * t17063;
    let t21372 = t21363 + t21364 + t21365 + t21366 + 2.0_f64 / 3.0_f64 * t11057 + (2e-21_f64 as f64) * t11060 + t11063 + 0.001515438175925926_f64 * t11065 + t11069 + t11073 + 0.18233333333333332_f64 * t11074 + t11079 + t11081 / 3.0_f64 + t11088;
    (t21363, t21364, t21365, t21366, t21372)
}
