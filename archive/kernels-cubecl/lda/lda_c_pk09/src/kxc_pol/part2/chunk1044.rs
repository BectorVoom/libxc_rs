//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1044/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1044<F: Float>(t10954: F, t10959: F, t10962: F, t10966: F, t11062: F, t11066: F, t11070: F, t11073: F, t11076: F, t6320: F, t6323: F, t6326: F, t6327: F, t6337: F, t6465: F, t6467: F) -> F {
    let t11351 = t6320 - F::cast_from(2.0_f64) * t6323 + t6326 + F::cast_from(2.0_f64) * t6327 - F::cast_from(2.0_f64) * t10954 + F::cast_from(4.0_f64) * t10959 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t10962 - F::cast_from(2.0_f64) * t10966 - F::cast_from(2.0_f64) * t11062 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6337 - t6465 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6467 + F::cast_from(2.0_f64) * t11066 - F::cast_from(2.0_f64) * t11070 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t11073 + F::cast_from(2.0_f64) * t11076;
    t11351
}
