//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1263/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1263<F: Float>(t132: F, t435: F, t6735: F, t16577: F, t16579: F, t16581: F, t16584: F, t16585: F, t16587: F, t16588: F, t16589: F, t16590: F, t16591: F, t16594: F, t16599: F, t16603: F, t9770: F) -> (F, F) {
    let t16605 = t132 * t435 * t6735;
    let t16606 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t16605;
    let t16607 = -t16577 - t16579 - t16581 - t16584 - t16585 - t16587 - t16588 - t16589 + t16590 + t16591 - t16594 - t9770 - t16599 - t16603 - t16606;
    (t16606, t16607)
}
