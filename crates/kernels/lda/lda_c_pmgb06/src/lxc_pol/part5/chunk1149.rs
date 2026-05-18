//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1149/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1149<F: Float>(t2960: F, t439: F, t477: F, t7481: F, t19791: F, t5260: F, t1901: F, t19754: F, t2010: F, t2002: F, t6376: F, t6379: F) -> (F, F, F, F, F) {
    let t20810 = F::new(2.0) / F::new(9.0) * t439 * t2960 * t7481 * t477;
    let t20813 = F::new(32.0) / F::new(27.0) * t439 * t5260 * t19791;
    let t20816 = F::new(4.0) / F::new(3.0) * t2010 * t1901 * t19754;
    let t20818 = F::new(2.0) / F::new(15.0) * t2002 * t6376;
    let t20820 = F::new(2.0) / F::new(5.0) * t2002 * t6379;
    (t20810, t20813, t20816, t20818, t20820)
}
