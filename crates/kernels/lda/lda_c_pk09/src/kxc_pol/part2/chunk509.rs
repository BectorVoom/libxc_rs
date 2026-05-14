//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 509/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk509<F: Float>(t3409: F, t3332: F, t3339: F, t3330: F, t14: F, t39: F, t36: F) -> (F, F, F, F, F, F) {
    let t3410 = 18.75 * t3409;
    let t3411 = 1.6622595016726363 * t3332;
    let t3412 = 0.2770432502787727 * t3339;
    let t3421 = 1.2466946262544771 * t3330;
    let t3422 = t39 * t14;
    let t3423 = t36 * t3422;
    (t3410, t3411, t3412, t3421, t3422, t3423)
}
