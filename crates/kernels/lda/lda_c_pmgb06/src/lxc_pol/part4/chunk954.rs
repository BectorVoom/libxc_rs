//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 954/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk954<F: Float>(t377: F, t5829: F, t2209: F, t374: F, t4232: F, t1773: F, t2262: F, t2266: F, t26: F, t4405: F, t4359: F, t5866: F, t5870: F, t297: F, t301: F, t413: F, t4463: F) -> (F, F, F, F, F, F, F, F) {
    let t11535 = t5829 * t377;
    let t11564 = t4232 * t2209 * t374;
    let t11567 = t1773 * t2262;
    let t11569 = t1773 * t2266;
    let t11574 = t4405 * t26;
    let t11583 = t4359 * t5866;
    let t11586 = t4359 * t5870;
    let t11596 = t297 * t4463 * t413 * t301;
    (t11535, t11564, t11567, t11569, t11574, t11583, t11586, t11596)
}
