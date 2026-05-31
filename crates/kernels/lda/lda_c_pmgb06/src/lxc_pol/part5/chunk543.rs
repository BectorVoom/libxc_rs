//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 543/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk543<F: Float>(t2801: F, t1139: F, t118: F, t415: F, t718: F, t1329: F, t1186: F, t1334: F, t421: F, t1147: F, t83: F) -> (F, F, F, F, F, F) {
    let t2802 = F::cast_from(24.0_f64) * t2801;
    let t2809 = t1139 * t118;
    let t2812 = F::cast_from(0.1890324433388467_f64) * t718 * t415;
    let t2816 = t1329 * t415;
    let t2820 = F::cast_from(0.01975389032890948_f64) * t1334 * t1186 * t421;
    let t2822 = t1147 * t83;
    (t2802, t2809, t2812, t2816, t2820, t2822)
}
