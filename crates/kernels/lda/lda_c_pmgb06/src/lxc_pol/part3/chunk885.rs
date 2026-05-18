//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 885/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk885<F: Float>(t1437: F, t1830: F, t455: F, t3100: F, t350: F, t1530: F, t3105: F, t132: F, t3121: F, t435: F, t161: F, t2944: F, t489: F) -> (F, F, F, F, F, F, F) {
    let t9189 = t1437 * t1437;
    let t9190 = F::new(1.0) / t9189;
    let t9215 = t1830 * t455;
    let t9217 = t350 * t3100;
    let t9220 = F::new(1.0) / t1437 / t1530;
    let t9225 = t350 * t3105;
    let t9234 = t132 * t435 * t3121;
    let t9237 = t161 * t489 * t2944;
    (t9190, t9215, t9217, t9220, t9225, t9234, t9237)
}
