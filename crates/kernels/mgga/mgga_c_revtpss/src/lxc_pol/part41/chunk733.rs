//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 733/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk733<F: Float>(t1082: F, t4757: F, t1089: F, t4905: F, t1651: F, t3291: F, t4772: F, t354: F, t357: F, t999: F, t4781: F, t3298: F, t378: F, t342: F, t3154: F, t3302: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4961 = t1082 * t4757;
    let t4964 = t4905 * t1089;
    let t4967 = t3291 * t1651;
    let t4970 = t1082 * t4772;
    let t4975 = t354 * t357;
    let t4976 = t4975 * t999;
    let t4977 = t4781 * t4976;
    let t4980 = t3298 * t378;
    let t4981 = t342 * t4980;
    let t4982 = t3302 * t3154;
    (t4961, t4964, t4967, t4970, t4976, t4977, t4980, t4981, t4982)
}
