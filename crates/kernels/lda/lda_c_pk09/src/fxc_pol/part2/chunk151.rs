//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 151/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk151<F: Float>(t435: F, t441: F, t467: F) -> (F, F) {
    let t470 = F::cast_from(0.7661514025603416_f64) * t435 + F::cast_from(0.3056501876701794_f64) * t441 + F::cast_from(0.03591334699501599_f64);
    let t471 = t467 * t470;
    (t470, t471)
}
