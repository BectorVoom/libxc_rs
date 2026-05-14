//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 481/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk481<F: Float>(t17: F, t24: F, t580: F, t68: F, t1146: F, t228: F, t21: F, t12: F, t567: F, t577: F, t590: F, t593: F, t576: F, t13: F, t18: F, t30: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3039 = t24 / t580 / t17 * t68;
    let t3040 = t228 * t1146;
    let t3041 = t3040 * t21;
    let t3044 = t12 * t567;
    let t3051 = t590 / t577 / t17 * t68;
    let t3056 = 1.0 / t593 / t17;
    let t3057 = t21 * t576 * t3056;
    let t3061 = t13 * t567;
    let t3065 = 1.0 / t18 / t17;
    let t3067 = t3065 * t68 * t228;
    let t3072 = t3044 * t30;
    (t3039, t3040, t3041, t3044, t3051, t3057, t3061, t3065, t3067, t3072)
}
