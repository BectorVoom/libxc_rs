//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 529/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk529<F: Float>(t179: F, t3233: F, t133: F, t3086: F, t131: F, t735: F, t567: F, t741: F, t743: F, t205: F, t568: F, t727: F, t728: F) -> (F, F, F, F, F) {
    let t3234 = t179 * t3233;
    let t3236 = t133 * t3086;
    let t3237 = t131 * t3236;
    let t3239 = F::cast_from(2.3693919160612835_f64) * t735 * t3237;
    let t3241 = t567 * t741;
    let t3242 = t3241 * t743;
    let t3243 = t205 * t3242;
    let t3246 = t727 * t568 * t728;
    (t3234, t3239, t3241, t3243, t3246)
}
