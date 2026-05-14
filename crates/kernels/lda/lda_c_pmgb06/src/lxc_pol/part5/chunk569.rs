//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 569/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk569<F: Float>(t1680: F, t581: F, t211: F, t410: F, t209: F, t1688: F, t591: F, t125: F, t208: F, t586: F) -> (F, F, F, F, F) {
    let t4102 = 2.0 / 9.0 * t581 * t1680;
    let t4103 = t211 * t410;
    let t4105 = 8.0 / 81.0 * t209 * t4103;
    let t4108 = t1688 * t591;
    let t4111 = t586 * t125 * t208;
    (t4102, t4103, t4105, t4108, t4111)
}
