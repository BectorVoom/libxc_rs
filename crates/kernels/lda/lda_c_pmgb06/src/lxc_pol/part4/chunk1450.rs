//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1450/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1450<F: Float>(t18542: F, t18566: F, t38: F, t56: F, t14816: F, t64: F, t365: F, t5772: F, t6996: F, t2703: F, t348: F, t110: F, t2209: F, t30: F) -> (F, F, F, F, F, F) {
    let t18568 = t18542 / F::new(2.0) + t18566 / F::new(2.0);
    let t18571 = F::new(2.923025) * t38 * t56 * t18568;
    let t18580 = F::new(11.6921) * t38 * t64 * t14816;
    let t18582 = t365 * t6996 * t5772;
    let t18585 = t348 * t2703 * t5772;
    let t18586 = F::new(5.84605) * t18585;
    let t18588 = t30 * t110 * t2209;
    (t18568, t18571, t18580, t18582, t18586, t18588)
}
