//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1339/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1339<F: Float>(t13515: F, t1438: F, t2106: F, t5083: F, t5086: F, t5108: F, t851: F, t1381: F, t5068: F, t12537: F, t13304: F, t17070: F) -> (F, F, F, F) {
    let t17593 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t13515;
    let t17597 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t5083 * t2106 * t1438 * t5086;
    let t17598 = t5108 * t851;
    let t17601 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t5068 * t17598 * t1381;
    let t17604 = F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t12537 * t13304 * t17070;
    (t17593, t17597, t17601, t17604)
}
