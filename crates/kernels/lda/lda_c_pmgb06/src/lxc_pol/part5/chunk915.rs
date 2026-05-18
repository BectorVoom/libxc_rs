//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 915/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk915<F: Float>(t11395: F, t2060: F, t5796: F, t5799: F, t947: F, t5802: F, t2221: F, t410: F, t360: F, t138: F, t53: F, t3631: F, t783: F) -> (F, F, F, F, F, F, F, F) {
    let t11396 = F::new(2.93808) * t11395;
    let t11398 = t5796 * t2060;
    let t11400 = t5799 * t947;
    let t11401 = F::new(1.9486833333333333) * t11400;
    let t11402 = t5802 * t2060;
    let t11404 = t410 * t2221;
    let t11405 = t360 * t11404;
    let t11406 = F::new(2.0) / F::new(3.0) * t11405;
    let t11407 = t53 * t138;
    let t11465 = t783 * t3631;
    (t11396, t11398, t11401, t11402, t11404, t11406, t11407, t11465)
}
