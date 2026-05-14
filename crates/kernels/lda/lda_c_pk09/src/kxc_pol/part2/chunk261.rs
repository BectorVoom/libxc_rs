//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 261/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk261<F: Float>(t44: F, t1196: F, t1197: F, t1189: F, t1193: F, t1195: F, t276: F, t1192: F, t51: F, zeta_threshold: F) -> (F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t1198 = t1196 * t1197;
    let t1201 = t1189 * t1193 + 1.28 * t1195 * t1198;
    let t1202 = t276 * t1201;
    let t1203 = piecewise3(t45, t1192, t1202);
    let t1204 = f64::ln(t51);
    (t1202, t1203, t1204)
}
