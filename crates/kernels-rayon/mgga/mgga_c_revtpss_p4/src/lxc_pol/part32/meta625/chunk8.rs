//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1986/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1986(t102477: f64, t102478: f64, t108531: f64, t108533: f64, t108535: f64, t108537: f64, t108539: f64, t108541: f64, t108543: f64, t108545: f64, t108547: f64, t108549: f64) -> f64 {
    let t109788 = -0.68598428988911579156e-2_f64 * t108531 + 0.34299214494455789578e-2_f64 * t108533 - 0.13719685797782315831e-1_f64 * t108535 - t102477 + t102478 + 7.0_f64 / 72.0_f64 * t108537 - 7.0_f64 / 24.0_f64 * t108539 - t108541 / 24.0_f64 - 0.34299214494455789578e-1_f64 * t108543 + 0.68598428988911579156e-2_f64 * t108545 + 0.68598428988911579156e-2_f64 * t108547 + 0.68598428988911579156e-2_f64 * t108549;
    t109788
}
