//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1193/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1193<F: Float>(t18848: F, t18851: F, t21411: F, t21439: F, t21442: F, t21445: F, t21448: F, t21451: F, t21461: F, t21462: F, t21463: F, t21465: F, t21466: F, t21477: F, t2247: F) -> F {
    let t21595 = F::cast_from(20.69106_f64) * t18848 - F::cast_from(10.34553_f64) * t18851 - F::cast_from(62.07318_f64) * t2247 * t21411 + t21439 - t21442 + t21445 + t21448 + t21451 - t21461 + t21462 + t21463 + t21465 - t21466 - t21477;
    t21595
}
