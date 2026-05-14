//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1076/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1076<F: Float>(t18274: F, t18277: F, t20465: F, t20467: F, t20472: F, t20478: F, t20480: F, t20482: F, t20486: F, t20490: F, t20491: F, t20492: F, t13140: F, t18281: F, t18284: F, t20493: F, t20495: F, t20497: F, t20499: F, t20501: F, t20503: F, t20504: F, t20505: F, t20506: F) -> (F, F) {
    let t21997 = -t20465 - t20467 - t20472 + t20478 - t20480 + t20482 - t20486 + t20490 - t20491 - t20492 + t18274 + 0.18233333333333332 * t18277;
    let t22000 = t18281 + 0.36466666666666664 * t18284 - t20493 - t20495 - t20497 - t20499 - t20501 - t20503 + t20504 - t20505 - t13140 + t20506;
    (t21997, t22000)
}
