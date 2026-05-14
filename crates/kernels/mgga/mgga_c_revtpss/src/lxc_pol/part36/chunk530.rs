//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 530/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk530<F: Float>(t1012: F, t3252: F, t140: F, t1655: F, t1011: F, t1678: F, t342: F, t1086: F, t1647: F, t3298: F, t378: F, t3154: F, t3302: F, t3316: F, t359: F, t198: F, t336: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t4919 = t1012 * t3252;
    let t4924 = t140 * t1655;
    let t4925 = t1011 * t4924;
    let t4935 = t342 * t1678;
    let t4954 = t1647 * t1086;
    let t4980 = t3298 * t378;
    let t4981 = t342 * t4980;
    let t4982 = t3302 * t3154;
    let t4995 = t3316 * t378;
    let t4996 = t342 * t4995;
    let t5004 = t359 * t1678;
    let t5023 = t198 * t336;
    (t4919, t4925, t4935, t4954, t4980, t4981, t4982, t4995, t4996, t5004, t5023)
}
