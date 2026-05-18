//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1342/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1342<F: Float>(t114484: F, t114513: F, t114611: F, t114632: F, t114664: F, t114701: F, t114718: F, t114740: F, t1450: F, t2014: F, t532: F, t196: F, t197: F, t22758: F) -> (F, F) {
    let t114746 = t2014 * t532 * (t114484 + t114513 + t114611 + t114632 + t114664 + t114701 + t114718 + t114740) * t1450;
    let t114752 = t22758 * t196 * t197;
    (t114746, t114752)
}
