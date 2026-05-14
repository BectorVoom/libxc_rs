//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1055/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1055<F: Float>(t1325: F, t3787: F, t4881: F, t5393: F, t519: F, t5359: F, t1519: F, t1982: F, t10722: F, t10725: F, t10729: F, t2072: F, t4073: F, t4647: F, t544: F, t1524: F, t2123: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t14343 = t1325 * t3787 * t4881;
    let t14344 = 16.0 / 15.0 * t14343;
    let t14346 = t1325 * t3787 * t5393;
    let t14347 = 8.0 / 15.0 * t14346;
    let t14349 = t519 * t3787 * t5359;
    let t14350 = 8.0 / 15.0 * t14349;
    let t14351 = t1982 * t1519;
    let t14352 = 4.0 / 45.0 * t14351;
    let t14353 = 8.0 / 15.0 * t10722;
    let t14354 = 8.0 / 15.0 * t10725;
    let t14355 = 16.0 / 45.0 * t10729;
    let t14357 = 4.0 / 5.0 * t4073 * t2072;
    let t14359 = 2.0 / 5.0 * t4647 * t544;
    let t14360 = t1524 * t2123;
    (t14344, t14347, t14350, t14352, t14353, t14354, t14355, t14357, t14359, t14360)
}
