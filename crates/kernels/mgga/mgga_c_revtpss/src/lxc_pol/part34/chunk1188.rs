//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1188/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1188<F: Float>(t114484: F, t114513: F, t114611: F, t114632: F, t114664: F, t114701: F, t114718: F, t114740: F, t1450: F, t2014: F, t532: F, t196: F, t197: F, t22758: F, t2035: F, t29499: F, t7898: F) -> (F, F, F) {
    let t114746 = t2014 * t532 * (t114484 + t114513 + t114611 + t114632 + t114664 + t114701 + t114718 + t114740) * t1450;
    let t114752 = t22758 * t196 * t197;
    let t114753 = t114752 * t2035;
    let t114755 = 18.0 * t7898 * t29499;
    (t114746, t114753, t114755)
}
