//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 519/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk519<F: Float>(t3104: F, t898: F, t905: F, t747: F, t838: F, t923: F, t748: F, t909: F, t913: F, t3103: F, t916: F, t919: F) -> (F, F, F, F, F, F, F, F) {
    let t3105 = t3104 * t898;
    let t3107 = t3104 * t905;
    let t3118 = t838 * t747;
    let t3119 = t3118 * t923;
    let t3120 = F::cast_from(29.43979784173208_f64) * t3119;
    let t3121 = t748 * t909;
    let t3123 = t748 * t913;
    let t3129 = t916 * t3103;
    let t3130 = t3129 * t919;
    (t3105, t3107, t3118, t3119, t3120, t3121, t3123, t3130)
}
