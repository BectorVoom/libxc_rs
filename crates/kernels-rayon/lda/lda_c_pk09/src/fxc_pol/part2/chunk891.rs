//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 891/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk891(t9353: f64, t9363: f64, t9375: f64, t9390: f64, t110: f64, t89: f64, t1124: f64, t2314: f64, t1094: f64, t1091: f64, t121: f64, t3141: f64) -> (f64, f64, f64) {
    let t9392 = t9353 + t9363 + t9375 + t9390;
    let t9393 = t110 * t9392;
    let t9394 = t9393 * t89;
    let t9408 = t2314 * t1124;
    let t9409 = t9408 * t1094;
    let t9410 = t121 * t1091;
    let t9411 = t3141 * t9410;
    (t9394, t9409, t9411)
}
