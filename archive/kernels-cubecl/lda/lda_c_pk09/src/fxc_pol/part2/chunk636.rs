//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 636/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk636<F: Float>(t1337: F, t5333: F, t131: F, t1350: F, t1348: F, t1369: F, t4998: F, t1345: F, t5081: F, t382: F, t5031: F, t5039: F) -> (F, F, F, F, F, F) {
    let t5335 = F::cast_from(0.027433775686566395_f64) * t1337 * t5333;
    let t5336 = t131 * t1350;
    let t5337 = t1348 * t5336;
    let t5340 = F::cast_from(12.423505345088643_f64) * t1369 * t4998;
    let t5341 = t1345 * t5081;
    let t5343 = t382 * t5031;
    let t5348 = F::cast_from(0.821419393556371_f64) * t5039;
    (t5335, t5337, t5340, t5341, t5343, t5348)
}
