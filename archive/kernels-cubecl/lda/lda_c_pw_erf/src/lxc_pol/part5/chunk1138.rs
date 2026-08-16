//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1138/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1138<F: Float>(t16069: F, t16072: F, t16075: F, t1318: F, t3854: F, t7679: F, t3863: F, t571: F, t7745: F, t3802: F, t519: F, t7741: F) -> (F, F, F, F, F, F) {
    let t21014 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t16069;
    let t21015 = F::cast_from(64.0_f64) / F::cast_from(81.0_f64) * t16072;
    let t21016 = F::cast_from(32.0_f64) / F::cast_from(27.0_f64) * t16075;
    let t21018 = t1318 * t3854 * t7679;
    let t21019 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t21018;
    let t21021 = t571 * t3863 * t7745;
    let t21022 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t21021;
    let t21024 = t519 * t3802 * t7741;
    (t21014, t21015, t21016, t21019, t21022, t21024)
}
