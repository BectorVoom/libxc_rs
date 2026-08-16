//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1051/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1051<F: Float>(t1083: F, t5070: F, t5068: F, t5069: F, t2872: F, t3458: F, t851: F, t2876: F, t5090: F, t12473: F, t12476: F, t12479: F, t12484: F, t12488: F, t12491: F, t12493: F, t12496: F, t12500: F) -> (F, F, F, F, F) {
    let t12501 = t5070 * t1083;
    let t12504 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t5068 * t5069 * t12501;
    let t12508 = F::cast_from(2.0_f64) / F::cast_from(5.0_f64) * t5068 * t3458 * t851 * t2872;
    let t12511 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t5068 * t5090 * t2876;
    let t12512 = t12473 - t12476 + t12479 + t12484 + t12488 - t12491 + t12493 - t12496 + t12500 + t12504 - t12508 - t12511;
    (t12501, t12504, t12508, t12511, t12512)
}
