//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 842/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk842<F: Float>(t2841: F, t4297: F, t4299: F, t4309: F, t707: F, t4313: F, t4317: F, t1100: F, t79: F, t2803: F, t297: F, t301: F, t413: F) -> (F, F, F, F, F, F, F) {
    let t8184 = F::cast_from(1.8276876377896586e-05_f64) * t4297 * t2841 * t4299;
    let t8185 = t707 * t4309;
    let t8187 = t707 * t4313;
    let t8189 = t707 * t4317;
    let t8193 = t79 * t1100;
    let t8194 = F::cast_from(120.0_f64) * t8193;
    let t8202 = t297 * t2803 * t413 * t301;
    (t8184, t8185, t8187, t8189, t8193, t8194, t8202)
}
