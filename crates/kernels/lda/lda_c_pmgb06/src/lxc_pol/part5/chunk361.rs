//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 361/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk361<F: Float>(t1521: F, t135: F, t146: F, t1568: F, t405: F, t474: F, t133: F, t134: F, t147: F) -> (F, F, F, F, F) {
    let t1607 = 0.047988888888888886 * t1521;
    let t1614 = 0.011111111111111112 * t146 * t1568 * t135;
    let t1615 = t405 * t474;
    let t1618 = 1.0 / t134 / t133;
    let t1619 = t147 * t1618;
    (t1607, t1614, t1615, t1618, t1619)
}
