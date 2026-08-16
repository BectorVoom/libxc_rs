//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2806/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2806<F: Float>(t10073: F, t10654: F, t10959: F, t2439: F, t2777: F, t10914: F, t2710: F, t9285: F, t10972: F, t2470: F, t874: F, t136: F, t2457: F, t2760: F) -> (F, F, F, F, F) {
    let t40924 = t10073 * t10654;
    let t40938 = t2439 * t2777 * t10959;
    let t40945 = t2710 * t10914 * t9285;
    let t40948 = t874 * t10972 * t2470;
    let t40952 = t2710 * t2760 * t136 * t2457;
    (t40924, t40938, t40945, t40948, t40952)
}
