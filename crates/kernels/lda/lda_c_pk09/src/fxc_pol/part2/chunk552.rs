//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 552/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk552<F: Float>(t3444: F, t62: F, t3422: F, t660: F, t665: F) -> (F, F, F) {
    let t3445 = F::cast_from(18.75_f64) * t3444;
    let t3446 = t62 * t62;
    let t3447 = F::cast_from(1.0_f64) / t3446;
    let t3452 = t660 * t3422;
    let t3453 = t3452 * t665;
    (t3445, t3447, t3453)
}
