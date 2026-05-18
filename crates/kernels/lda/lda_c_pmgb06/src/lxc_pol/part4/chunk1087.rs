//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1087/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1087<F: Float>(t130: F, t431: F, t5076: F, t1414: F, t1601: F, t1908: F, t3213: F, t464: F, t4779: F, t1387: F, t5220: F, t161: F, t489: F, t4936: F) -> (F, F, F, F, F, F, F) {
    let t12683 = t431 * t130;
    let t12684 = t12683 * t5076;
    let t12691 = t1601 * t1414;
    let t12752 = t3213 * t1908;
    let t12772 = t4779 * t464;
    let t12784 = t5220 * t1387;
    let t12787 = t161 * t489 * t4936;
    (t12683, t12684, t12691, t12752, t12772, t12784, t12787)
}
