//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 87/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk87<F: Float>(t43: F, t6: F, t259: F) -> (F, F, F, F, F) {
    let t267 = 2.0970850588349075 * t43;
    let t268 = 2.5218818358694817 * t6;
    let t269 = 1.0 * t259;
    let t270 = -0.5778610319323944 + t267 - t268 + t269;
    let t271 = 1.0 / t270;
    (t267, t268, t269, t270, t271)
}
