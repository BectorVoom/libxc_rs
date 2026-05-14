//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 844/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk844<F: Float>(t549: F, t593: F, t1318: F, t3837: F, t3854: F, t1620: F, t598: F, t226: F, t4232: F, t1159: F, t603: F, t1634: F, t695: F, t2070: F, t493: F, t495: F) -> (F, F, F, F, F, F, F) {
    let t10392 = t549 * t593;
    let t10403 = t1318 * t3854 * t3837;
    let t10409 = t598 * t1620;
    let t10412 = 16.0 / 3.0 * t226 * t4232;
    let t10414 = t1159 * t603;
    let t10417 = 0.004413481481481482 * t695 * t1634;
    let t10419 = t493 * t2070 * t495;
    (t10392, t10403, t10409, t10412, t10414, t10417, t10419)
}
