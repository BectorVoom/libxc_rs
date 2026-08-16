//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1149/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1149<F: Float>(t13715: F, t3262: F, t439: F, t1969: F, t3213: F, t1423: F, t4620: F, t5197: F, t5202: F, t1886: F, t607: F, t446: F) -> (F, F, F, F, F) {
    let t13718 = F::cast_from(8.0_f64) / F::cast_from(81.0_f64) * t439 * t13715 * t3262;
    let t13719 = t3213 * t1969;
    let t13720 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t13719;
    let t13721 = t1423 * t4620;
    let t13722 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t13721;
    let t13725 = F::cast_from(2.0_f64) / F::cast_from(5.0_f64) * t439 * t5197 * t5202;
    let t13726 = t1886 * t607;
    let t13727 = t13726 * t446;
    (t13718, t13720, t13722, t13725, t13727)
}
