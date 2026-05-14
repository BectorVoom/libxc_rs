//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 329/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk329<F: Float>(t118: F, t1329: F, t391: F, t415: F, t1179: F, t83: F, t419: F, t421: F, t399: F, t398: F, t409: F, t117: F, t1184: F) -> (F, F, F, F, F, F, F, F) {
    let t1330 = t1329 * t118;
    let t1333 = 0.06301081444628223 * t391 * t415;
    let t1334 = t1179 * t83;
    let t1337 = 0.006584630109636494 * t1334 * t419 * t421;
    let t1341 = t399 * t415;
    let t1343 = t409 * t398;
    let t1345 = t1343 * t419 * t421;
    let t1347 = t1184 * t117;
    (t1330, t1333, t1334, t1337, t1341, t1343, t1345, t1347)
}
