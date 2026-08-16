//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 516/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk516(t17: f64, t577: f64, t590: f64, t68: f64, t593: f64, t21: f64, t576: f64, t13: f64, t567: f64, t18: f64, t228: f64, t30: f64, t3044: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3051 = t590 / t577 / t17 * t68;
    let t3056 = 1.0_f64 / t593 / t17;
    let t3057 = t21 * t576 * t3056;
    let t3061 = t13 * t567;
    let t3065 = 1.0_f64 / t18 / t17;
    let t3067 = t3065 * t68 * t228;
    let t3072 = t3044 * t30;
    (t3051, t3057, t3061, t3065, t3067, t3072)
}
