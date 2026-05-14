//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1166/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1166<F: Float>(t13515: F, t1438: F, t2106: F, t5083: F, t5086: F, t5108: F, t851: F, t1381: F, t5068: F, t12537: F, t13304: F, t17070: F, t17457: F, t5139: F, t13068: F, t5138: F) -> (F, F, F, F, F, F) {
    let t17593 = 8.0 / 45.0 * t13515;
    let t17597 = 4.0 / 27.0 * t5083 * t2106 * t1438 * t5086;
    let t17598 = t5108 * t851;
    let t17601 = 8.0 / 45.0 * t5068 * t17598 * t1381;
    let t17604 = 16.0 / 9.0 * t12537 * t13304 * t17070;
    let t17607 = 4.0 / 15.0 * t5068 * t5139 * t17457;
    let t17610 = 4.0 / 9.0 * t5138 * t13068 * t17457;
    (t17593, t17597, t17601, t17604, t17607, t17610)
}
