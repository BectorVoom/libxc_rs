//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1221/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1221(t187: f64, t5632: f64, t3389: f64, t856: f64, t5635: f64, t5638: f64, t13985: f64, t13988: f64, t13992: f64, t13995: f64, t13997: f64, t13999: f64, t14002: f64) -> f64 {
    let t14465 = t5632 * t187;
    let t14467 = t856 * t3389;
    let t14469 = t5635 * t187;
    let t14471 = t5638 * t187;
    let t14472 = 8.0_f64 * t14471;
    let t14473 = t13985 - t13988 - t13992 + 4.0_f64 * t14465 + 0.0011033703703703704_f64 * t14467 + 4.0_f64 * t14469 + t14472 - t13995 - t13997 - t13999 - t14002;
    t14473
}
