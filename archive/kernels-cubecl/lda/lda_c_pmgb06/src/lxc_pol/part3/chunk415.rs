//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 415/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk415<F: Float>(t1555: F, t161: F, t153: F, t1531: F, t1069: F, t442: F) -> (F, F, F, F) {
    let t1557 = t161 * t1555 / F::cast_from(135.0_f64);
    let t1558 = t153 * t1531;
    let t1559 = t1558 * t1069;
    let t1560 = t442 * t1559;
    (t1557, t1558, t1559, t1560)
}
