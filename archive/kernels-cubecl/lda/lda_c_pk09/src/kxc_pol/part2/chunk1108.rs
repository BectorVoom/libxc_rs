//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1108/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1108<F: Float>(t10954: F, t10962: F, t10966: F, t11062: F, t11070: F, t11556: F, t11559: F, t11563: F, t11566: F, t11574: F, t6327: F, t6637: F, t6639: F, t7446: F, t7453: F, t7454: F, t7459: F) -> F {
    let t12275 = t6639 - t6637 - F::cast_from(4.0_f64) * t11556 + F::cast_from(4.0_f64) * t11559 - F::cast_from(0.2738064645187903_f64) * t10962 + F::cast_from(2.6666666666666665_f64) * t11563 - F::cast_from(2.6666666666666665_f64) * t11566 - F::cast_from(0.821419393556371_f64) * t11070 - F::cast_from(0.821419393556371_f64) * t10954 - F::cast_from(0.821419393556371_f64) * t10966 - F::cast_from(0.821419393556371_f64) * t11062 + F::cast_from(4.0_f64) * t11574 + F::cast_from(0.821419393556371_f64) * t6327 + t7446 + t7453 - t7454 - t7459;
    t12275
}
