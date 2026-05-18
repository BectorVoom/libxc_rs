//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 768/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk768<F: Float>(t477: F, t760: F, t332: F, t5084: F, t5083: F, t1601: F, t851: F, t1381: F, t5068: F, t1531: F, t465: F) -> (F, F, F, F, F, F, F, F) {
    let t5085 = t760 * t477;
    let t5086 = t5085 * t332;
    let t5087 = t5084 * t5086;
    let t5089 = F::new(2.0) / F::new(27.0) * t5083 * t5087;
    let t5090 = t1601 * t851;
    let t5091 = t5090 * t1381;
    let t5093 = F::new(4.0) / F::new(45.0) * t5068 * t5091;
    let t5094 = t465 * t1531;
    (t5085, t5086, t5087, t5089, t5090, t5091, t5093, t5094)
}
