//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 601/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk601<F: Float>(t1322: F, t3416: F, t1287: F, t558: F, t352: F, t1319: F, t1318: F, t1320: F, t954: F, t1351: F, t549: F, t951: F) -> (F, F, F, F, F, F, F, F) {
    let t3418 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t3416 * t1322;
    let t3419 = t1287 * t558;
    let t3420 = t3419 * t352;
    let t3421 = t1319 * t3420;
    let t3423 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t1318 * t3421;
    let t3424 = t1320 * t954;
    let t3425 = t1319 * t3424;
    let t3427 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t1318 * t3425;
    let t3429 = t549 * t1351 * t951;
    (t3418, t3420, t3421, t3423, t3424, t3425, t3427, t3429)
}
