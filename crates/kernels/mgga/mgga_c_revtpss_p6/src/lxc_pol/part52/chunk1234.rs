//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1234/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1234<F: Float>(t109269: F, t32578: F, t27833: F, t8718: F, t32626: F, t7901: F, t34328: F, t7235: F, t651: F, t7002: F, t8065: F, t128266: F, t128270: F, t128273: F, t128274: F, t128277: F, t2007: F, t28652: F, t28707: F, t7221: F, t7969: F, t8568: F) -> F {
    let t128279 = F::cast_from(2.0_f64) * t109269 * t32578;
    let t128280 = t27833 * t8718;
    let t128282 = F::cast_from(3.0_f64) * t32626 * t7901;
    let t128284 = t7235 * t34328;
    let t128287 = F::cast_from(2.0_f64) * t651 * t8065 * t7002;
    let t128288 = -t2007 * t28652 - t28707 * t8568 - t7221 * t7969 - t128266 + t128270 - t128273 - t128274 + t128277 + t128279 - t128280 + t128282 - t128284 - t128287;
    t128288
}
