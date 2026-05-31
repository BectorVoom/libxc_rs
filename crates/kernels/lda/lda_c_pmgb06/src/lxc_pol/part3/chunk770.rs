//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 770/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk770<F: Float>(t446: F, t5220: F, t1420: F, t1898: F, t1426: F, t153: F, t1864: F, t439: F, t1387: F, t2002: F, t2064: F, t443: F) -> (F, F, F, F, F, F, F) {
    let t5222 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t5220 * t446;
    let t5224 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t1420 * t1898;
    let t5225 = t1426 * t153;
    let t5226 = t5225 * t1864;
    let t5228 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t439 * t5226;
    let t5230 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t2002 * t1387;
    let t5231 = t2064 * t443;
    (t5222, t5224, t5225, t5226, t5228, t5230, t5231)
}
