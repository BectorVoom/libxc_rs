//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2619/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2619<F: Float>(t14923: F, t18428: F, t10760: F, t40627: F, t61837: F, t18527: F, t50295: F, t18353: F, t2689: F, t18394: F, t2703: F, t10777: F, t14686: F, t61715: F, t837: F) -> (F, F, F, F, F, F) {
    let t62108 = t14923 * t18428;
    let t62111 = t10760 * t40627 * t61837;
    let t62114 = t50295 * t18527;
    let t62129 = t2689 * t18353;
    let t62135 = t2703 * t18394;
    let t62148 = t10777 * t14686 * t61715 * t837;
    (t62108, t62111, t62114, t62129, t62135, t62148)
}
