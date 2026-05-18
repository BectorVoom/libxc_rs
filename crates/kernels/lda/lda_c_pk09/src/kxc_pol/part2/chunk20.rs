//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 20/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk20<F: Float>(t44: F, zeta_threshold: F) -> (F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t46 = pow_1_3::<f64>(zeta_threshold);
    let t47 = pow_1_3::<f64>(t44);
    let t48 = piecewise3::<f64>(t45, t46, t47);
    (t46, t47, t48)
}
