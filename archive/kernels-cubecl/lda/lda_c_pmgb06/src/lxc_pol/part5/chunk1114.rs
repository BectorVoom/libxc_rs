//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1114/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1114<F: Float>(t16749: F, t1995: F, t6134: F, t165: F, t1994: F, t2553: F, t493: F, t136: F, t1968: F, t2582: F, t439: F, t529: F, t7621: F) -> (F, F, F, F, F) {
    let t20380 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t16749;
    let t20382 = t6134 * t1995 / F::cast_from(5.0_f64);
    let t20386 = t493 * t165 * t2553 * t1994 / F::cast_from(5.0_f64);
    let t20390 = t439 * t136 * t2582 * t1968 / F::cast_from(5.0_f64);
    let t20391 = t7621 * t529;
    (t20380, t20382, t20386, t20390, t20391)
}
