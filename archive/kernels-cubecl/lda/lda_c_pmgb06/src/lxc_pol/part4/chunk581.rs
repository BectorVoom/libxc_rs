//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 581/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk581<F: Float>(t2000: F, t2016: F, t2019: F, t1464: F, t2386: F) -> (F, F, F, F) {
    let t2534 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t2000;
    let t2535 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t2016;
    let t2536 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t2019;
    let t2541 = t1464 * t2386;
    (t2534, t2535, t2536, t2541)
}
