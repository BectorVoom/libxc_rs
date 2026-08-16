//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 906/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk906(t5049: f64, t56: f64, t1214: f64, t2474: f64, t57: f64) -> (f64, f64) {
    let t9625 = t5049 * t56;
    let t9626 = t2474 * t1214;
    let t9627 = t57 * t9626;
    let t9628 = t9625 * t9627;
    (t9626, t9628)
}
