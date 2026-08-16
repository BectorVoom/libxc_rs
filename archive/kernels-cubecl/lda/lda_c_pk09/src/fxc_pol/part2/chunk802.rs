//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 802/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk802<F: Float>(t161: F, t164: F, t3483: F, t3485: F, t3488: F, t3490: F, t3497: F, t3500: F, t3744: F, t3758: F, t8046: F, t8049: F, t8053: F, t8061: F, t8066: F, t8069: F, t8073: F) -> F {
    let t8076 = F::cast_from(18.635258017632964_f64) * t8046 + F::cast_from(4.937333717448355_f64) * t161 * t8049 - F::cast_from(0.04115066352984959_f64) * t164 * t8053 + F::cast_from(1.4760499452555382_f64) * t3483 - F::cast_from(12.423505345088643_f64) * t3485 - F::cast_from(12.992782516386768_f64) * t3488 + F::cast_from(1.8805371096875316_f64) * t3490 + t3497 + t3500 - F::cast_from(1.1846959580306418_f64) * t3744 * t8061 + F::cast_from(4.738783832122567_f64) * t8066 + F::cast_from(4.738783832122567_f64) * t3758 * t8069 + F::cast_from(4.738783832122567_f64) * t3758 * t8073;
    t8076
}
