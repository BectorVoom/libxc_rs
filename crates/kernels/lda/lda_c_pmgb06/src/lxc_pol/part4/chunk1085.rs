//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1085/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1085<F: Float>(t1423: F, t5233: F, t4588: F, t517: F, t1925: F, t3223: F, t5238: F, t1908: F, t3220: F, t1382: F, t5194: F, t1592: F, t1962: F) -> (F, F, F, F, F, F, F) {
    let t12603 = t1423 * t5233;
    let t12617 = t4588 * t517;
    let t12621 = t3223 * t1925;
    let t12623 = t1423 * t5238;
    let t12625 = t3220 * t1908;
    let t12631 = t5194 * t1382;
    let t12633 = t1962 * t1592;
    (t12603, t12617, t12621, t12623, t12625, t12631, t12633)
}
