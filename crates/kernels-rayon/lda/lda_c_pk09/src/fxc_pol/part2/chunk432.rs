//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 432/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk432(t2171: f64, t2175: f64, t2179: f64, t613: f64, t617: f64, t188: f64, t2192: f64, t659: f64, t702: f64, t89: f64, t2143: f64, t61: f64) -> (f64, f64, f64, f64, f64) {
    let t2233 = t613 + t617 + 0.9421211958699838_f64 * t2171 + 0.9421211958699838_f64 * t2175 - 0.9421211958699838_f64 * t2179;
    let t2237 = t2233 * t188 - t659 * t2192 / 2.0_f64;
    let t2238 = t2237 * t702;
    let t2239 = t2238 * t89;
    let t2246 = t61 * t2143;
    (t2233, t2237, t2238, t2239, t2246)
}
