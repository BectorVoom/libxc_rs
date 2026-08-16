//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1341/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1341<F: Float>(t1386: F, t17617: F, t5077: F, t2599: F, t3458: F, t1381: F, t5068: F, t5090: F, t5493: F, t2604: F, t3032: F, t5078: F, t5232: F) -> (F, F, F, F, F) {
    let t17620 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t5077 * t17617 * t1386;
    let t17621 = t3458 * t2599;
    let t17624 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t5068 * t17621 * t1381;
    let t17627 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t5068 * t5090 * t5493;
    let t17628 = t3032 * t2604;
    let t17631 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t5077 * t17628 * t1386;
    let t17634 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t5077 * t5078 * t5232;
    (t17620, t17624, t17627, t17631, t17634)
}
