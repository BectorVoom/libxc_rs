//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2180/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2180<F: Float>(t22299: F, t26028: F, t22093: F, t22098: F, t108531: F, t108533: F, t108535: F, t108537: F, t108539: F, t108541: F, t108543: F, t98129: F, t98131: F) -> F {
    let t108545 = t26028 * t22299;
    let t108547 = t26028 * t22093;
    let t108549 = t26028 * t22098;
    let t108551 = -F::cast_from(0.34299214494455789578e-2_f64) * t108531 + F::cast_from(0.17149607247227894789e-2_f64) * t108533 - F::cast_from(0.68598428988911579156e-2_f64) * t108535 - t98129 + t98131 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t108537 - F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t108539 - t108541 / F::cast_from(48.0_f64) - F::cast_from(0.17149607247227894789e-1_f64) * t108543 + F::cast_from(0.34299214494455789578e-2_f64) * t108545 + F::cast_from(0.34299214494455789578e-2_f64) * t108547 + F::cast_from(0.34299214494455789578e-2_f64) * t108549;
    t108551
}
