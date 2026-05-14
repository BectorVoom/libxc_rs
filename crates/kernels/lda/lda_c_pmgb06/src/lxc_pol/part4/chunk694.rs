//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 694/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk694<F: Float>(t4757: F, t493: F, t1602: F, t3457: F, t851: F, t1992: F, t1594: F, t3031: F, t822: F, t1966: F, t439: F, t1417: F, t1972: F, t1559: F, t1962: F, t1560: F, t2002: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t4759 = 2.0 / 45.0 * t493 * t4757;
    let t4761 = t3457 * t851 * t1602;
    let t4762 = t1992 * t4761;
    let t4764 = t493 * t4762 / 5.0;
    let t4766 = t3031 * t822 * t1594;
    let t4767 = t1966 * t4766;
    let t4769 = t439 * t4767 / 5.0;
    let t4771 = 2.0 / 45.0 * t1972 * t1417;
    let t4772 = t1962 * t1559;
    let t4774 = 2.0 / 45.0 * t439 * t4772;
    let t4776 = 2.0 / 45.0 * t2002 * t1560;
    (t4759, t4761, t4762, t4764, t4766, t4767, t4769, t4771, t4772, t4774, t4776)
}
