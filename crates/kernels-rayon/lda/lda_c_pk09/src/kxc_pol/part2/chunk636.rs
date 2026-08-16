//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 636/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk636(t1337: f64, t5333: f64, t131: f64, t1350: f64, t1348: f64, t1369: f64, t4998: f64, t1345: f64, t5081: f64, t382: f64, t5031: f64, t5039: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5335 = 0.027433775686566395_f64 * t1337 * t5333;
    let t5336 = t131 * t1350;
    let t5337 = t1348 * t5336;
    let t5340 = 12.423505345088643_f64 * t1369 * t4998;
    let t5341 = t1345 * t5081;
    let t5343 = t382 * t5031;
    let t5348 = 0.821419393556371_f64 * t5039;
    (t5335, t5337, t5340, t5341, t5343, t5348)
}
