//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1112/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1112<F: Float>(t12296: F, t2956: F, t2957: F, t2958: F, t2959: F, t4: F, t7566: F) -> F {
    let tv2rho21 = t12296 * t4 + t2956 + t2957 + t2958 + F::new(2.0) * t2959 + F::new(2.0) * t7566;
    tv2rho21
}
