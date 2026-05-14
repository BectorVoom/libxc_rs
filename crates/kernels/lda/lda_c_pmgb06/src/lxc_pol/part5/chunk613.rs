//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 613/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk613<F: Float>(t107: F, t1180: F, t902: F, t1795: F, t566: F, t1329: F, t868: F, t1808: F, t391: F, t247: F, t794: F, t113: F, t301: F, t1147: F, t123: F, t317: F) -> (F, F, F, F, F, F, F) {
    let t5529 = t107 * t1180 * t902;
    let t5542 = 0.1675256410710088 * t1795 * t566;
    let t5551 = 0.1675256410710088 * t1329 * t868;
    let t5553 = 0.1675256410710088 * t391 * t1808;
    let t5567 = t247 * t794;
    let t5569 = t5567 * t113 * t301;
    let t5573 = t123 * t1147 * t902 * t317;
    (t5529, t5542, t5551, t5553, t5567, t5569, t5573)
}
