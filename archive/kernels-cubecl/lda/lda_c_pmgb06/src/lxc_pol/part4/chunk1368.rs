//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1368/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1368<F: Float>(t17960: F, t14211: F, t14213: F, t1600: F, t6904: F, t1992: F, t493: F, t529: F, t5179: F, t6113: F, t1420: F, t6255: F) -> (F, F, F, F, F, F) {
    let t17961 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t17960;
    let t17962 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t14211;
    let t17963 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t14213;
    let t17964 = t1600 * t6904;
    let t17968 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t493 * t1992 * t17964 * t529;
    let t17971 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t493 * t5179 * t6113;
    let t17973 = F::cast_from(2.0_f64) / F::cast_from(5.0_f64) * t1420 * t6255;
    (t17961, t17962, t17963, t17968, t17971, t17973)
}
