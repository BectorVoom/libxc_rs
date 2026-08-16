//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 676/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk676<F: Float>(t3785: F, t667: F, t273: F, t3738: F, t3703: F, t3741: F, t3709: F, t967: F, t409: F, t675: F, t109: F, t963: F) -> (F, F, F, F, F, F, F) {
    let t3800 = t3785 * t667;
    let t3803 = t273 * t3738;
    let t3804 = t3703 * t3741;
    let t3807 = t273 * t3709;
    let t3808 = t3703 * t967;
    let t3811 = t409 * t675;
    let t3818 = t109 * t963;
    (t3800, t3803, t3804, t3807, t3808, t3811, t3818)
}
