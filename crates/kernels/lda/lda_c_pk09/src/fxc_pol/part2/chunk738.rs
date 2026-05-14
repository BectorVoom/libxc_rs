//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 738/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk738<F: Float>(t3317: F, t3335: F, t3342: F, t3599: F, t3601: F, t7801: F, t7805: F, t7809: F, t7811: F, t7814: F, t7817: F, t7834: F, t3319: F, t3323: F, t3326: F, t3598: F, t7896: F, t7919: F, t7923: F, t7926: F, t7928: F, t7931: F, t7935: F, t7939: F, t7942: F) -> (F, F) {
    let t8360 = -4.0 / 3.0 * t7801 - 2.0 * t7805 - 2.0 * t7809 - 2.0 * t7811 - 2.0 * t7814 - 2.0 * t7817 - 2.0 * t7834 - 2.0 * t3335 - 4.0 / 3.0 * t3342 + t3599 - t3601 + 2.0 * t3317;
    let t8373 = 2.0 * t3319 + 4.0 / 3.0 * t3323 + 4.0 / 3.0 * t3326 + t3598 + 4.0 * t7896 + 2.0 * t7919 + 2.0 * t7923 + 2.0 * t7926 + 2.0 * t7928 + 2.0 * t7931 + 2.0 * t7935 + 4.0 / 3.0 * t7939 + 4.0 / 3.0 * t7942;
    (t8360, t8373)
}
