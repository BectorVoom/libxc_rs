//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 973/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk973<F: Float>(t4317: F, t707: F, t1100: F, t79: F, t1166: F, t1183: F, t297: F, t301: F, t4320: F, t711: F, t715: F, t1227: F, t27: F, t402: F) -> (F, F, F, F, F, F, F) {
    let t8189 = t707 * t4317;
    let t8193 = t79 * t1100;
    let t8194 = F::new(120.0) * t8193;
    let t8206 = t297 * t1166 * t1183 * t301;
    let t8208 = t4320 * t711;
    let t8211 = F::new(0.7805426614091894) * t4320 * t715;
    let t8228 = t1227 * t27 * t402;
    (t8189, t8193, t8194, t8206, t8208, t8211, t8228)
}
