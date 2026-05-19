//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 80/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk80<F: Float>(t215: F, t8: F, t18: F, t19: F, t9: F) -> (F, F, F) {
    let t216 = t215 * t8;
    let t217 = t18 * t18;
    let t221 = F::exp(-F::cast_from(0.1173961225190475_f64) * t19);
    let t225 = F::cast_from(0.41081146652128_f64) + F::cast_from(0.14983581422587874_f64) * t216 * t217 + F::cast_from(0.01928080210487025_f64) * t221 * t9 * t18;
    (t217, t221, t225)
}
