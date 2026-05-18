//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 652/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk652<F: Float>(t186: F, t409: F, t55: F, t543: F, t1400: F, t27: F, t545: F, t1403: F, t534: F, t97: F, t1377: F, t1410: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3309 = t55 * t409 * t186;
    let t3311 = F::new(0.09618703433213194) * t543 * t3309;
    let t3312 = t1400 * t27;
    let t3313 = t3312 * t545;
    let t3315 = t1403 * t27;
    let t3316 = t3315 * t545;
    let t3319 = t534 * t97;
    let t3320 = t3319 * t1377;
    let t3322 = t1410 * t27;
    (t3309, t3311, t3312, t3313, t3315, t3316, t3319, t3320, t3322)
}
