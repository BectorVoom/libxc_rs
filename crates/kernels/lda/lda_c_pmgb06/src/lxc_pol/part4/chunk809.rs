//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 809/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk809<F: Float>(t1525: F, t6146: F, t36: F, t3090: F, t6151: F, t6155: F, t1830: F, t2570: F, t332: F, t453: F, t1: F, t1863: F, t473: F, t6160: F, t1619: F, t6165: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6176 = t1525 * t6146;
    let t6177 = t36 * t6176;
    let t6179 = t3090 * t6151;
    let t6180 = t36 * t6179;
    let t6182 = t1525 * t6155;
    let t6183 = t1830 * t6182;
    let t6185 = t2570 * t332;
    let t6186 = t453 * t6185;
    let t6187 = t36 * t6186;
    let t6189 = t1863 * t1;
    let t6190 = t453 * t6189;
    let t6191 = t1830 * t6190;
    let t6193 = t473 * t6185;
    let t6196 = t473 * t6189;
    let t6199 = t473 * t6160;
    let t6202 = t1619 * t6165;
    (t6176, t6177, t6179, t6180, t6182, t6183, t6185, t6186, t6187, t6189, t6190, t6191, t6193, t6196, t6199, t6202)
}
