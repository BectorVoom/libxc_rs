//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 862/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk862<F: Float>(t7888: F, t3323: F, t3326: F, t3706: F, t3713: F, t3715: F, t3961: F, t3962: F, t3963: F, t7870: F, t7875: F, t7879: F, t7884: F) -> F {
    let t8926 = F::cast_from(24.0_f64) * t7888;
    let t8927 = F::cast_from(0.5476129290375806_f64) * t3323 + F::cast_from(0.5476129290375806_f64) * t3326 + t3706 + F::cast_from(24.0_f64) * t7870 - F::cast_from(24.0_f64) * t7875 + F::cast_from(24.0_f64) * t7879 - F::cast_from(24.0_f64) * t7884 + t8926 + t3961 + t3962 - t3963 + t3713 + t3715;
    t8927
}
