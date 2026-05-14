//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 911/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk911<F: Float>(t12232: F, t2852: F, t802: F, t3134: F, t824: F, t1554: F, t161: F, t2100: F, t3043: F, t831: F, t3461: F, t3450: F, t132: F, t435: F, t4965: F, t432: F, t5120: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12233 = 4.0 / 405.0 * t12232;
    let t12234 = t802 * t2852;
    let t12235 = 4.0 / 405.0 * t12234;
    let t12237 = t3134 * t824 / 30.0;
    let t12239 = t161 * t1554 * t2100;
    let t12240 = t12239 / 45.0;
    let t12241 = t831 * t3043;
    let t12242 = 2.0 / 15.0 * t12241;
    let t12244 = t831 * t3461 / 5.0;
    let t12245 = t831 * t3450;
    let t12246 = t12245 / 45.0;
    let t12248 = t132 * t435 * t4965;
    let t12249 = t12248 / 15.0;
    let t12251 = t432 * t5120 / 5.0;
    (t12233, t12235, t12237, t12240, t12242, t12244, t12246, t12249, t12251)
}
