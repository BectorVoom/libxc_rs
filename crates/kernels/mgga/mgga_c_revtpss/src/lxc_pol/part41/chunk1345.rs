//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1345/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1345<F: Float>(t31328: F, t575: F, t1921: F, t8283: F, t1455: F, t8389: F, t31619: F, t571: F, t2184: F, t6951: F, t2192: F, t6936: F, t117369: F, t117374: F, t117772: F, t117774: F, t1464: F, t1914: F, t2185: F, t22571: F, t31377: F, t31583: F, t5790: F, t5808: F, t8284: F, t8373: F) -> (F, F, F, F) {
    let t117783 = 2.0 * t31328 * t575;
    let t117789 = 2.0 * t8283 * t1921;
    let t117793 = 2.0 * t1455 * t8389;
    let t118208 = t571 * t31619;
    let t118209 = t2184 * t6951;
    let t118213 = t6936 * t2192;
    let t118217 = t1464 * t31583 + 2.0 * t1914 * t31377 + t2185 * t22571 + 2.0 * t5790 * t8389 + 2.0 * t5808 * t8373 + t6951 * t8284 + t117369 + t117374 + t117772 + t117774 + t118208 + t118209 + t118213;
    (t117783, t117789, t117793, t118217)
}
