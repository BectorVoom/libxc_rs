//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 429/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk429<F: Float>(t1593: F, t1594: F, t137: F, t132: F, t175: F, t516: F) -> (F, F, F, F) {
    let t1595 = t1593 * t1594;
    let t1596 = t137 * t1595;
    let t1598 = t132 * t1596 / F::new(15.0);
    let t1600 = F::new(1.0) / t516 / t175;
    (t1595, t1596, t1598, t1600)
}
