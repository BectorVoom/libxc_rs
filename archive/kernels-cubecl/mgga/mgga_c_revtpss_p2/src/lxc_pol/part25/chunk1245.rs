//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1245/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1245<F: Float>(t92977: F, t93018: F, t93057: F, t93097: F, t231: F, t92883: F, t10073: F, t25308: F, t25403: F, t25402: F, t7048: F, t7056: F) -> (F, F, F, F) {
    let t93099 = t92977 + t93018 + t93057 + t93097;
    let t93104 = t92883 * t231;
    let t93112 = t10073 * t25308 * t25403;
    let t93116 = t10073 * t7056 * t25402 * t7048;
    (t93099, t93104, t93112, t93116)
}
