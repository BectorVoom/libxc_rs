//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 251/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk251<F: Float>(t44: F, t1127: F, t776: F, t879: F, t938: F, t7: F, t620: F, t413: F, t13: F, t236: F, t229: F, t243: F, t567: F, t9: F, t235: F, t4: F, t72: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t1129 = t776 + t879 + t938 + t1127;
    let t1130 = t7 * t1129;
    let t1134 = piecewise3(t45, 0.0, 2.0 * t44 * t620);
    let t1135 = t1134 * t413;
    let t1137 = t13 * t236;
    let t1139 = t229 * t243;
    let t1142 = t9 * t567;
    let t1143 = t235 * t1142;
    let t1146 = 1.0 / t72 / t4;
    (t1129, t1130, t1134, t1135, t1137, t1139, t1142, t1143, t1146)
}
