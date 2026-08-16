//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 432/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk432<F: Float>(t1601: F, t1602: F, t166: F, t161: F, t1521: F, t135: F, t146: F, t1568: F, t405: F, t474: F, t133: F, t134: F) -> (F, F, F, F, F, F, F) {
    let t1603 = t1601 * t1602;
    let t1604 = t166 * t1603;
    let t1606 = t161 * t1604 / F::cast_from(15.0_f64);
    let t1607 = F::cast_from(0.047988888888888886_f64) * t1521;
    let t1614 = F::cast_from(0.011111111111111112_f64) * t146 * t1568 * t135;
    let t1615 = t405 * t474;
    let t1618 = F::cast_from(1.0_f64) / t134 / t133;
    (t1603, t1604, t1606, t1607, t1614, t1615, t1618)
}
