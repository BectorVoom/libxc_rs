//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 886/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk886<F: Float>(t1179: F, t161: F, t165: F, t177: F, t3279: F, t464: F, t1450: F, t1600: F, t3031: F, t458: F, t3457: F, t511: F) -> (F, F, F, F, F) {
    let t10134 = F::new(28.0) / F::new(1215.0) * t161 * t1179 * t165 * t177;
    let t10148 = t3279 * t464;
    let t10152 = t1450 * t1600;
    let t10178 = t458 * t3031;
    let t10185 = t511 * t3457;
    (t10134, t10148, t10152, t10178, t10185)
}
