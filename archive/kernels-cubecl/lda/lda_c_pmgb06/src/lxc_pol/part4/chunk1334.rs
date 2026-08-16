//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1334/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1334<F: Float>(t2623: F, t3457: F, t1602: F, t1992: F, t493: F, t27: F, t545: F, t7209: F, t7179: F, t1377: F, t2676: F, t97: F) -> (F, F, F, F) {
    let t17538 = t3457 * t2623;
    let t17542 = t493 * t1992 * t17538 * t1602 / F::cast_from(5.0_f64);
    let t17544 = t7209 * t27 * t545;
    let t17547 = t7179 * t27 * t545;
    let t17550 = t2676 * t97 * t1377;
    (t17542, t17544, t17547, t17550)
}
