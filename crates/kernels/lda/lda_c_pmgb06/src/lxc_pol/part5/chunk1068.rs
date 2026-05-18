//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1068/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1068<F: Float>(t1830: F, t19782: F, t453: F, t332: F, t7477: F, t1525: F, t36: F, t7284: F, t9220: F, t3090: F, t19471: F, t9188: F) -> (F, F, F, F, F, F) {
    let t19784 = t1830 * t453 * t19782;
    let t19786 = t7477 * t332;
    let t19788 = t36 * t1525 * t19786;
    let t19791 = t9220 * t7284 * t332;
    let t19793 = t36 * t3090 * t19791;
    let t19796 = t36 * t9188 * t19471;
    (t19784, t19786, t19788, t19791, t19793, t19796)
}
