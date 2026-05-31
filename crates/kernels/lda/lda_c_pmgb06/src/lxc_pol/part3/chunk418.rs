//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 418/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk418<F: Float>(t1586: F, t518: F, t166: F, t161: F, t152: F, t463: F) -> (F, F, F, F) {
    let t1587 = t518 * t1586;
    let t1588 = t166 * t1587;
    let t1590 = t161 * t1588 / F::cast_from(30.0_f64);
    let t1592 = F::cast_from(1.0_f64) / t463 / t152;
    (t1587, t1588, t1590, t1592)
}
