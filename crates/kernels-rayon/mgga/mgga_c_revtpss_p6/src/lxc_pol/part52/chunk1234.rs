//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1234/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1234(t109269: f64, t32578: f64, t27833: f64, t8718: f64, t32626: f64, t7901: f64, t34328: f64, t7235: f64, t651: f64, t7002: f64, t8065: f64, t128266: f64, t128270: f64, t128273: f64, t128274: f64, t128277: f64, t2007: f64, t28652: f64, t28707: f64, t7221: f64, t7969: f64, t8568: f64) -> f64 {
    let t128279 = 2.0_f64 * t109269 * t32578;
    let t128280 = t27833 * t8718;
    let t128282 = 3.0_f64 * t32626 * t7901;
    let t128284 = t7235 * t34328;
    let t128287 = 2.0_f64 * t651 * t8065 * t7002;
    let t128288 = -t2007 * t28652 - t28707 * t8568 - t7221 * t7969 - t128266 + t128270 - t128273 - t128274 + t128277 + t128279 - t128280 + t128282 - t128284 - t128287;
    t128288
}
