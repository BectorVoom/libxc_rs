//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 823/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk823<F: Float>(t1390: F, t1449: F, t3807: F, t519: F, t3762: F, t581: F, t1309: F, t571: F, t3828: F, t3863: F, t1325: F, t3818: F, t3859: F, t3794: F, t3860: F, t3675: F, t522: F) -> (F, F, F, F, F, F, F, F) {
    let t9304 = t1449 * t1390;
    let t9306 = t519 * t9304 * t3807;
    let t9313 = t3762 * t581;
    let t9315 = t571 * t9313 * t1309;
    let t9318 = t571 * t3863 * t3828;
    let t9338 = t1325 * t3859 * t3818;
    let t9340 = t3794 * t3860;
    let t9351 = t522 * t3675;
    (t9304, t9306, t9313, t9315, t9318, t9338, t9340, t9351)
}
