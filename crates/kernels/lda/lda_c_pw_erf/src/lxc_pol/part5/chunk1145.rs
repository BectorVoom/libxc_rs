//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1145/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1145<F: Float>(t15685: F, t6265: F, t3863: F, t571: F, t7709: F, t3854: F, t7426: F, t558: F, t7836: F, t1318: F, t1319: F, t352: F) -> (F, F, F, F) {
    let t21093 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t15685 * t6265;
    let t21095 = t571 * t3863 * t7709;
    let t21096 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t21095;
    let t21098 = t571 * t3854 * t7426;
    let t21099 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t21098;
    let t21100 = t7836 * t558;
    let t21104 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t1318 * t1319 * t21100 * t352;
    (t21093, t21096, t21099, t21104)
}
