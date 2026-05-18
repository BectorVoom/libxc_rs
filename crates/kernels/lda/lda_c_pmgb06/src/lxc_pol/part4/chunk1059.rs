//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1059/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1059<F: Float>(t11404: F, t360: F, t138: F, t53: F, t3631: F, t783: F, t34: F, t3615: F, t109: F, t1282: F, t2247: F, t5875: F) -> (F, F, F, F, F, F) {
    let t11405 = t360 * t11404;
    let t11407 = t53 * t138;
    let t11465 = t783 * t3631;
    let t11470 = t34 * t3615;
    let t11475 = t109 * t1282;
    let t11477 = t2247 * t11475 * t5875;
    (t11405, t11407, t11465, t11470, t11475, t11477)
}
