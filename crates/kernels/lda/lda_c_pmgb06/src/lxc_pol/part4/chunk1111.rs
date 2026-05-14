//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1111/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1111<F: Float>(t16593: F, t517: F, t6831: F, t161: F, t166: F, t529: F, t1586: F, t6230: F, t132: F, t435: F, t6735: F, t16577: F, t16579: F, t16581: F, t16584: F, t16585: F, t16587: F, t16588: F, t16589: F, t16590: F, t16591: F, t9770: F) -> (F, F, F, F, F) {
    let t16594 = t16593 / 135.0;
    let t16595 = t6831 * t517;
    let t16599 = t161 * t166 * t16595 * t529 / 15.0;
    let t16603 = t161 * t166 * t6230 * t1586 / 30.0;
    let t16605 = t132 * t435 * t6735;
    let t16606 = 2.0 / 45.0 * t16605;
    let t16607 = -t16577 - t16579 - t16581 - t16584 - t16585 - t16587 - t16588 - t16589 + t16590 + t16591 - t16594 - t9770 - t16599 - t16603 - t16606;
    (t16594, t16599, t16603, t16606, t16607)
}
