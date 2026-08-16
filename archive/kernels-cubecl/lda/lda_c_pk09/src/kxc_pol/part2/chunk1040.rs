//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1040/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1040<F: Float>(t11277: F, t2042: F, t10: F, t1729: F, t549: F, t11059: F, t132: F, t93: F, t1672: F, t2872: F, t11248: F, t1808: F) -> (F, F, F, F, F) {
    let t11278 = t11277 * t2042;
    let t11282 = t1729 * t10;
    let t11283 = t549 * t11282;
    let t11286 = t132 * t11059;
    let t11287 = t93 * t11286;
    let t11290 = t2872 * t1672;
    let t11292 = t1808 * t11248;
    (t11278, t11283, t11287, t11290, t11292)
}
