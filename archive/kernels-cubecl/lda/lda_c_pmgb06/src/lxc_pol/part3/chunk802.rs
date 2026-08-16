//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 802/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk802<F: Float>(t123: F, t2164: F, t317: F, t740: F, t117: F, t2360: F, t315: F, t1179: F, t794: F, t419: F, t421: F, t1798: F, t409: F) -> (F, F, F, F, F) {
    let t5601 = F::cast_from(0.10809180959278285_f64) * t123 * t740 * t2164 * t317;
    let t5610 = F::cast_from(0.017961351015381915_f64) * t123 * t315 * t2360 * t117;
    let t5613 = t1179 * t794;
    let t5615 = t5613 * t419 * t421;
    let t5617 = t409 * t1798;
    (t5601, t5610, t5613, t5615, t5617)
}
