//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 949/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk949<F: Float>(t14310: F, t1366: F, t5655: F, t2349: F, t3309: F, t3333: F, t5649: F, t1377: F, t2342: F, t97: F, t2345: F, t27: F, t545: F, t5638: F) -> (F, F, F, F, F, F, F) {
    let t14311 = F::new(0.21642082724729686) * t14310;
    let t14312 = t5655 * t1366;
    let t14314 = t2349 * t3309;
    let t14316 = t5649 * t3333;
    let t14347 = t2342 * t97 * t1377;
    let t14348 = F::new(0.03354522822333102) * t14347;
    let t14350 = t2345 * t97 * t1377;
    let t14356 = t5638 * t27 * t545;
    (t14311, t14312, t14314, t14316, t14348, t14350, t14356)
}
