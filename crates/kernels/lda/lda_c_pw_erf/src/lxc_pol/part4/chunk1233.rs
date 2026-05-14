//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1233/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1233<F: Float>(t18311: F, t3854: F, t571: F, t6361: F, t4794: F, t6366: F, t1472: F, t6357: F, t1308: F, t593: F, t6665: F, t1381: F, t2419: F, t6362: F, t1319: F, t16330: F) -> (F, F, F, F, F, F, F, F) {
    let t18312 = 16.0 / 135.0 * t18311;
    let t18314 = t571 * t3854 * t6361;
    let t18315 = 32.0 / 135.0 * t18314;
    let t18317 = t571 * t4794 * t6366;
    let t18318 = 16.0 / 81.0 * t18317;
    let t18320 = 8.0 / 45.0 * t1472 * t6357;
    let t18324 = 8.0 / 45.0 * t571 * t1308 * t6665 * t593;
    let t18328 = 4.0 / 45.0 * t571 * t1308 * t2419 * t1381;
    let t18330 = 16.0 / 45.0 * t1472 * t6362;
    let t18333 = 16.0 / 45.0 * t571 * t1319 * t16330;
    (t18312, t18315, t18318, t18320, t18324, t18328, t18330, t18333)
}
