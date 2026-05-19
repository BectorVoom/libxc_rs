//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 925/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk925<F: Float>(t1139: F, t1183: F, t301: F, t1100: F, t83: F, t113: F, t2778: F, t413: F, t398: F, t642: F, t1234: F, t384: F) -> (F, F, F, F, F, F, F) {
    let t10635 = t1139 * t1183 * t301;
    let t10637 = t1100 * t83;
    let t10640 = F::cast_from(0.03831185177913979_f64) * t10637 * t113 * t301;
    let t10643 = F::cast_from(0.026861343269868797_f64) * t2778 * t413 * t301;
    let t10644 = t642 * t398;
    let t10646 = t10644 * t113 * t301;
    let t10648 = t384 * t1234;
    (t10635, t10637, t10640, t10643, t10644, t10646, t10648)
}
