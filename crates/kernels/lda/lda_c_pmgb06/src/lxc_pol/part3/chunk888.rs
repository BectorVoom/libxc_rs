//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 888/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk888<F: Float>(t3043: F, t486: F, t1387: F, t3213: F, t1423: F, t2957: F, t1683: F, t1730: F, t1687: F, t4159: F, t573: F, t580: F) -> (F, F, F, F, F, F, F) {
    let t9317 = t486 * t3043;
    let t9330 = t3213 * t1387;
    let t9332 = t1423 * t2957;
    let t9338 = t1683 * t1730;
    let t9340 = t1687 * t1730;
    let t9342 = t573 * t4159;
    let t9345 = F::new(0.26596355555555556) * t580 * t4159;
    (t9317, t9330, t9332, t9338, t9340, t9342, t9345)
}
