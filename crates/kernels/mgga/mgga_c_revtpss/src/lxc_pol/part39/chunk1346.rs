//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1346/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1346<F: Float>(t2184: F, t5808: F, t31328: F, t575: F, t1921: F, t8283: F, t1455: F, t8389: F, t116899: F, t117090: F, t117097: F, t117099: F, t117713: F, t1456: F, t18217: F, t1914: F, t2185: F, t3: F, t31127: F, t31377: F, t8284: F) -> (F,) {
    let t117781 = 2.0 * t2184 * t5808;
    let t117783 = 2.0 * t31328 * t575;
    let t117789 = 2.0 * t8283 * t1921;
    let t117793 = 2.0 * t1455 * t8389;
    let t117796 = t117713 * t3 * t575 + 2.0 * t1456 * t31377 + t18217 * t2185 + t1914 * t31127 + 2.0 * t5808 * t8284 + 2.0 * t116899 + t117090 + t117097 + 2.0 * t117099 + t117781 + t117783 + t117789 + t117793;
    (t117796,)
}
