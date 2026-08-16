//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 817/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk817(t8192: f64, t8202: f64, t8214: f64, t8229: f64, t188: f64, t2192: f64, t3928: f64, t3933: f64, t659: f64, t694: f64, t8169: f64, t8171: f64, t8176: f64) -> (f64, f64) {
    let t8231 = t8192 + t8202 + t8214 + t8229;
    let t8234 = t8169 * t188 - t8171 * t694 / 2.0_f64 - t3928 * t2192 / 2.0_f64 + 3.0_f64 / 4.0_f64 * t3933 * t8176 - t659 * t8231 / 2.0_f64;
    (t8231, t8234)
}
