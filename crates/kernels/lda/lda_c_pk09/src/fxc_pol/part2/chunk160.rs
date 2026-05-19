//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 160/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk160<F: Float>(t429: F, t507: F, t435: F, t441: F) -> (F, F, F) {
    let t508 = t507 * t429;
    let t513 = F::new(2.0) * t435 + F::cast_from(0.821419393556371_f64) * t441 + F::cast_from(0.10532352447676886_f64);
    let t514 = F::ln(t513);
    (t508, t513, t514)
}
