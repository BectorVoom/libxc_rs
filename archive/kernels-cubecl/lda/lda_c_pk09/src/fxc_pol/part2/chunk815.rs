//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 815/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk815<F: Float>(t3323: F, t3326: F, t3424: F, t3426: F, t3428: F, t3960: F, t3967: F, t3969: F, t7870: F, t7875: F, t7879: F, t7884: F, t7888: F) -> F {
    let t8214 = F::cast_from(0.337177226155986_f64) * t3323 + F::cast_from(0.337177226155986_f64) * t3326 + t3960 + F::cast_from(12.0_f64) * t7870 - F::cast_from(12.0_f64) * t7875 + F::cast_from(12.0_f64) * t7879 - F::cast_from(12.0_f64) * t7884 + F::cast_from(12.0_f64) * t7888 + F::cast_from(8.0_f64) * t3424 + F::cast_from(8.0_f64) * t3426 - F::cast_from(8.0_f64) * t3428 + t3967 + t3969;
    t8214
}
