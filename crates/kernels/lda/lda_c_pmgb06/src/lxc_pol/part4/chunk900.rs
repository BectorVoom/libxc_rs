//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 900/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk900<F: Float>(t337: F, t6516: F, t2871: F, t493: F, t6474: F, t6477: F, t6480: F, t6482: F, t6484: F, t6486: F, t6488: F, t6490: F, t6493: F, t6497: F, t6501: F, t6506: F, t6511: F, t6515: F) -> (F, F, F, F) {
    let t6517 = t6516 * t337;
    let t6518 = t2871 * t6517;
    let t6520 = F::new(2.0) / F::new(45.0) * t493 * t6518;
    let t6521 = -t6474 + t6477 + t6480 - t6482 + t6484 - t6486 - t6488 + t6490 - t6493 - t6497 + t6501 - t6506 + t6511 - t6515 + t6520;
    (t6517, t6518, t6520, t6521)
}
