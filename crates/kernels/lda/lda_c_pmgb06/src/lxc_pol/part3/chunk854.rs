//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 854/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk854<F: Float>(t1289: F, t342: F, t4232: F, t1311: F, t26: F, t329: F, t1035: F, t1041: F, t1043: F, t3947: F, t687: F, t217: F) -> (F, F, F, F, F, F, F) {
    let t8470 = t4232 * t1289 * t342;
    let t8473 = t26 * t1311;
    let t8474 = t329 * t8473;
    let t8479 = t1035 * t1035;
    let t8482 = F::cast_from(48.245938496077606_f64) * t1041 * t8479 * t1043;
    let t8483 = t3947 * t687;
    let t8485 = F::cast_from(1.0_f64) / t217;
    (t8470, t8473, t8474, t8479, t8482, t8483, t8485)
}
