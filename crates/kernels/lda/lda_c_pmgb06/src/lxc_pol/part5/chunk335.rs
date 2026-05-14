//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 335/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk335<F: Float>(t435: F, t478: F, t132: F, t458: F, t464: F, t398: F, t539: F, t188: F, t947: F, t955: F) -> (F, F, F, F, F, F) {
    let t1392 = t435 * t478;
    let t1393 = t132 * t1392;
    let t1395 = t458 * t464;
    let t1403 = t398 * t539;
    let t1404 = t1403 * t188;
    let t1409 = -0.55 * t947 + 5.0 / 18.0 * t955;
    (t1392, t1393, t1395, t1403, t1404, t1409)
}
