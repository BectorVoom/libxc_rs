//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1004/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1004<F: Float>(t365: F, t5772: F, t6996: F, t2703: F, t348: F, t110: F, t2209: F, t30: F, t5783: F, t360: F, t7031: F, t2707: F) -> (F, F, F, F, F, F) {
    let t18582 = t365 * t6996 * t5772;
    let t18585 = t348 * t2703 * t5772;
    let t18588 = t30 * t110 * t2209;
    let t18589 = t5783 * t18588;
    let t18609 = t360 * t110 * t7031;
    let t18615 = t348 * t2707 * t5772;
    (t18582, t18585, t18588, t18589, t18609, t18615)
}
