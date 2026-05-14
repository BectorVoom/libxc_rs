//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 856/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk856<F: Float>(t14347: F, t1377: F, t2345: F, t97: F, t27: F, t545: F, t5638: F, t5632: F, t187: F, t3389: F, t856: F, t1409: F, t1798: F, t188: F, t3023: F, t794: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14348 = 0.03354522822333102 * t14347;
    let t14350 = t2345 * t97 * t1377;
    let t14356 = t5638 * t27 * t545;
    let t14357 = 0.6492624817418906 * t14356;
    let t14359 = t5632 * t27 * t545;
    let t14465 = t5632 * t187;
    let t14467 = t856 * t3389;
    let t14471 = t5638 * t187;
    let t14472 = 8.0 * t14471;
    let t14481 = t1798 * t1409 * t188;
    let t14482 = 4.0 * t14481;
    let t14484 = t794 * t3023 * t188;
    (t14348, t14350, t14357, t14359, t14465, t14467, t14472, t14482, t14484)
}
