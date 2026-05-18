//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 737/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk737<F: Float>(t44: F, t4752: F, t131: F, t155: F, t1416: F, t1988: F, t493: F, t1602: F, t3457: F, t851: F, t1992: F, t1594: F, t3031: F, t822: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4753 = t4752 * t44;
    let t4754 = t4753 * t131;
    let t4756 = t4754 * t155 / F::new(30.0);
    let t4757 = t1988 * t1416;
    let t4759 = F::new(2.0) / F::new(45.0) * t493 * t4757;
    let t4761 = t3457 * t851 * t1602;
    let t4762 = t1992 * t4761;
    let t4764 = t493 * t4762 / F::new(5.0);
    let t4766 = t3031 * t822 * t1594;
    (t4753, t4754, t4756, t4757, t4759, t4761, t4762, t4764, t4766)
}
