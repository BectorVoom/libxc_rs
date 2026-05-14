//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 919/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk919<F: Float>(t12156: F, t36: F, t9188: F, t12161: F, t1830: F, t3090: F, t1863: F, t3115: F, t453: F, t12165: F, t3010: F, t4649: F, t350: F, t4651: F, t139: F, t30: F, t35: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12377 = t36 * t9188 * t12156;
    let t12380 = t1830 * t3090 * t12161;
    let t12382 = t1863 * t3115;
    let t12384 = t36 * t453 * t12382;
    let t12387 = t1830 * t453 * t12165;
    let t12389 = t4649 * t3010;
    let t12391 = t36 * t453 * t12389;
    let t12393 = t350 * t4651;
    let t12396 = t30 * t35 * t139;
    (t12377, t12380, t12382, t12384, t12387, t12389, t12391, t12393, t12396)
}
