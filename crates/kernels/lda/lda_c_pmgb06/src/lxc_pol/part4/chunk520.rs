//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 520/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk520<F: Float>(t5: F, t1905: F, t1986: F, t2038: F, t2114: F, t107: F, t410: F, t902: F, t1068: F, t760: F, t1: F, t9: F, t332: F, t395: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t2116 = t1905 + t1986 + t2038 + t2114;
    let t2122 = t107 * t410 * t902;
    let t2125 = t1068 * t760;
    let t2128 = t9 * t1;
    let t2132 = piecewise3::<f64>(t6, F::new(0.0), F::new(4.0) / F::new(9.0) * t2125 * t332 + F::new(8.0) / F::new(3.0) * t2128 * t395);
    (t2116, t2122, t2125, t2128, t2132)
}
