//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 620/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk620<F: Float>(t2801: F, t1338: F, t415: F, t1139: F, t118: F, t718: F, t1166: F, t81: F, t1329: F, t1186: F, t1334: F, t421: F) -> (F, F, F, F, F, F, F, F) {
    let t2802 = F::new(24.0) * t2801;
    let t2807 = t1338 * t415;
    let t2809 = t1139 * t118;
    let t2812 = F::cast_from(0.1890324433388467_f64) * t718 * t415;
    let t2813 = t81 * t1166;
    let t2814 = t2813 * t118;
    let t2816 = t1329 * t415;
    let t2820 = F::cast_from(0.01975389032890948_f64) * t1334 * t1186 * t421;
    (t2802, t2807, t2809, t2812, t2813, t2814, t2816, t2820)
}
