//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 917/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk917<F: Float>(t44: F, t6703: F, t131: F, t155: F, t2592: F, t460: F, t1928: F, t802: F, t2029: F, t4111: F, t2802: F, t4461: F, t4462: F) -> (F, F, F, F, F, F, F) {
    let t6704 = t6703 * t44;
    let t6705 = t6704 * t131;
    let t6707 = t6705 * t155 / F::cast_from(30.0_f64);
    let t6709 = t2592 * t460 / F::cast_from(30.0_f64);
    let t6710 = t802 * t1928;
    let t6711 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t6710;
    let t6715 = F::cast_from(2e-21_f64) * t2029 * t4111;
    let t6716 = -t4461 + t4462 + t2802;
    (t6704, t6705, t6707, t6709, t6711, t6715, t6716)
}
