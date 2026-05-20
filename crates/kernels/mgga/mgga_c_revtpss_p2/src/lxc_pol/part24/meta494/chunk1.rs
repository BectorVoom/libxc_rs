//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1494/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1494<F: Float>(t22335: F, t2470: F, t4101: F, t10073: F, t22361: F, t10069: F, t22373: F, t10139: F, t136: F, t2457: F, t6874: F, t6844: F) -> (F, F, F, F, F) {
    let t75092 = t4101 * t22335 * t2470;
    let t75113 = t10073 * t22361;
    let t75119 = t10069 * t22373;
    let t75123 = t10139 * t6874 * t136 * t2457;
    let t75128 = t10139 * t6844 * t136 * t2457;
    (t75092, t75113, t75119, t75123, t75128)
}
