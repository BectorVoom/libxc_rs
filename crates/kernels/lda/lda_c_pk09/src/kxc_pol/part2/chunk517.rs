//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 517/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk517<F: Float>(t3523: F, t3525: F, t3118: F, t841: F, t155: F, t3230: F, t3233: F, t1067: F, t976: F, t14: F, t257: F, t130: F, t133: F, t128: F, t1146: F, t94: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3527 = 0.08230132705969918 * t3523 * t3525;
    let t3529 = 0.05486755137313279 * t3118 * t841;
    let t3534 = t155 * t3230;
    let t3536 = t155 * t3233;
    let t3538 = t976 * t1067;
    let t3551 = 1.0 / t14 / t257;
    let t3552 = t3551 * t130;
    let t3553 = t3552 * t133;
    let t3554 = t128 * t3553;
    let t3555 = 2.800127110638765 * t3554;
    let t3556 = t1146 * t94;
    (t3527, t3529, t3534, t3536, t3538, t3551, t3553, t3554, t3555, t3556)
}
