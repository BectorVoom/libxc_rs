//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 635/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk635<F: Float>(t1289: F, t377: F, t1295: F, t374: F, t376: F, t67: F, t1180: F, t56: F, t69: F, t3530: F, t3533: F, t3585: F, t1112: F, t974: F, t1039: F, t620: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3622 = t1289 * t377;
    let t3625 = t374 * t1295;
    let t3630 = t376 * t376;
    let t3631 = 1.0 / t3630;
    let t3632 = t67 * t3631;
    let t3643 = 0.8940581481481481 * t69 * t1180 * t56;
    let t3644 = t69 * t3530;
    let t3646 = t69 * t3533;
    let t3654 = t69 * t3585;
    let t3662 = t974 * t1112;
    let t3665 = 1.0 / t1039 / t620;
    (t3622, t3625, t3630, t3631, t3632, t3643, t3644, t3646, t3654, t3662, t3665)
}
