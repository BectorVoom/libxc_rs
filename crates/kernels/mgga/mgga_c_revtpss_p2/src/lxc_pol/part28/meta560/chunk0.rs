//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2017/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2017<F: Float>(t10073: F, t25402: F, t7048: F, t7056: F, t233: F, t41077: F, t25348: F, t689: F, t25411: F, t1955: F, t92888: F, t9646: F) -> (F, F, F, F, F, F) {
    let t93116 = t10073 * t7056 * t25402 * t7048;
    let t93118 = t41077 * t233;
    let t93123 = t25348 * t689;
    let t93124 = t25411 * t93123;
    let t93126 = t1955 * t92888;
    let t93134 = t9646 * t7056;
    (t93116, t93118, t93123, t93124, t93126, t93134)
}
