//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 971/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk971<F: Float>(t11211: F, t11222: F, t11225: F, t11229: F, t11236: F, t11289: F, t11470: F, t11477: F, t11485: F, t11488: F, t11491: F, t1234: F, t2209: F, t2247: F, t3588: F, t5874: F, t769: F, t8428: F) -> F {
    let t11493 = t11211 - t11222 + t11225 + t11229 - t11236 + t8428 - t11289 + F::cast_from(103.4553_f64) * t2247 * t11470 * t769 * t3588 + F::cast_from(20.69106_f64) * t11477 - F::cast_from(62.07318_f64) * t2247 * t5874 * t2209 * t1234 + F::cast_from(6.89702_f64) * t11485 - F::cast_from(10.34553_f64) * t11488 - F::cast_from(5.172765_f64) * t11491;
    t11493
}
