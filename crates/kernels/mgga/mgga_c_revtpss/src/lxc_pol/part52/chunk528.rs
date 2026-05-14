//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 528/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk528<F: Float>(t1651: F, t3291: F, t1082: F, t4772: F, t354: F, t357: F, t999: F, t4781: F, t3298: F, t378: F, t342: F, t3154: F, t3302: F, t1043: F, t4893: F, t1071: F, t1089: F, t1668: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4967 = t3291 * t1651;
    let t4970 = t1082 * t4772;
    let t4975 = t354 * t357;
    let t4976 = t4975 * t999;
    let t4977 = t4781 * t4976;
    let t4980 = t3298 * t378;
    let t4981 = t342 * t4980;
    let t4982 = t3302 * t3154;
    let t4983 = t4982 * t1043;
    let t4984 = t4893 * t4983;
    let t4988 = t1071 * t1668 * t1089;
    (t4967, t4970, t4975, t4976, t4977, t4981, t4983, t4984, t4988)
}
