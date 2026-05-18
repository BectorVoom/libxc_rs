//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 985/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk985<F: Float>(t2813: F, t868: F, t107: F, t410: F, t4575: F, t122: F, t4182: F, t886: F, t5508: F, t569: F, t199: F, t5567: F) -> (F, F, F, F, F) {
    let t11720 = t2813 * t868;
    let t11723 = t107 * t410 * t4575;
    let t11726 = t122 * t4182 * t886;
    let t11729 = t122 * t569 * t5508;
    let t11731 = t5567 * t199;
    (t11720, t11723, t11726, t11729, t11731)
}
