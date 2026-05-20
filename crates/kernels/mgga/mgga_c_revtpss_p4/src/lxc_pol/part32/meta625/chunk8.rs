//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1986/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1986<F: Float>(t102477: F, t102478: F, t108531: F, t108533: F, t108535: F, t108537: F, t108539: F, t108541: F, t108543: F, t108545: F, t108547: F, t108549: F) -> F {
    let t109788 = -F::cast_from(0.68598428988911579156e-2_f64) * t108531 + F::cast_from(0.34299214494455789578e-2_f64) * t108533 - F::cast_from(0.13719685797782315831e-1_f64) * t108535 - t102477 + t102478 + F::new(7.0) / F::new(72.0) * t108537 - F::new(7.0) / F::new(24.0) * t108539 - t108541 / F::new(24.0) - F::cast_from(0.34299214494455789578e-1_f64) * t108543 + F::cast_from(0.68598428988911579156e-2_f64) * t108545 + F::cast_from(0.68598428988911579156e-2_f64) * t108547 + F::cast_from(0.68598428988911579156e-2_f64) * t108549;
    t109788
}
