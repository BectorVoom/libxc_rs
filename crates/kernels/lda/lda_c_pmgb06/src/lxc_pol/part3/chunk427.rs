//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 427/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk427<F: Float>(t1629: F, t465: F, t137: F, t132: F, t486: F, t531: F, t489: F, t530: F) -> (F, F, F, F, F) {
    let t1630 = t465 * t1629;
    let t1631 = t137 * t1630;
    let t1633 = t132 * t1631 / F::new(30.0);
    let t1635 = t486 * t531 / F::new(15.0);
    let t1636 = t489 * t530;
    (t1630, t1631, t1633, t1635, t1636)
}
