//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1128/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1128<F: Float>(t12691: F, t16888: F, t5068: F, t13064: F, t5138: F, t13177: F, t1083: F, t2871: F, t493: F, t6516: F, t1464: F, t2599: F, t10139: F, t1080: F, t6507: F, t5470: F) -> (F, F, F, F, F, F, F) {
    let t16891 = 16.0 / 45.0 * t5068 * t12691 * t16888;
    let t16894 = 8.0 / 27.0 * t5138 * t13064 * t16888;
    let t16895 = 16.0 / 1215.0 * t13177;
    let t16899 = 2.0 / 45.0 * t493 * t2871 * t6516 * t1083;
    let t16900 = t2599 * t1464;
    let t16904 = 2.0 / 27.0 * t493 * t10139 * t16900 * t1080;
    let t16905 = t6507 * t1083;
    let t16908 = 8.0 / 81.0 * t493 * t5470 * t16905;
    (t16891, t16894, t16895, t16899, t16904, t16905, t16908)
}
