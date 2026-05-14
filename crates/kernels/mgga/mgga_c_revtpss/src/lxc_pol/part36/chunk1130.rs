//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1130/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1130<F: Float>(t17416: F, t7624: F, t17376: F, t26843: F, t26848: F, t17400: F, t26866: F, t1802: F, t3089: F, t3717: F, t1285: F, t5326: F, t7623: F, t17523: F, t26842: F, t3594: F, sigma2: F) -> (F, F, F, F, F, F, F, F) {
    let t104658 = t7624 * t17416;
    let t104682 = t17376 * t26843;
    let t104685 = t17376 * t26848;
    let t104703 = t17400 * t26866;
    let t104706 = sigma2 * t1802;
    let t104707 = t104706 * t3089;
    let t104708 = t3717 * t104707;
    let t104721 = t1285 * t104707;
    let t104752 = t5326 * t7623;
    let t104758 = t3594 * t26842 * t17523;
    (t104658, t104682, t104685, t104703, t104708, t104721, t104752, t104758)
}
