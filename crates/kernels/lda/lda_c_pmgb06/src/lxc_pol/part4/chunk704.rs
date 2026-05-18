//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 704/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk704<F: Float>(t297: F, t4317: F, t122: F, t4182: F, t302: F, t1773: F, t715: F, t711: F, t325: F, t326: F, t327: F, t312: F) -> (F, F, F, F, F, F, F) {
    let t4318 = t297 * t4317;
    let t4320 = t122 * t4182;
    let t4322 = F::new(0.19513566535229734) * t4320 * t302;
    let t4324 = F::new(0.15965645347006147) * t1773 * t715;
    let t4325 = t1773 * t711;
    let t4343 = F::new(1.0) / t327 / t326 / t325;
    let t4344 = t4343 * t312;
    (t4318, t4320, t4322, t4324, t4325, t4343, t4344)
}
