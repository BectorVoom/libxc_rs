//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2155/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2155<F: Float>(t20020: F, t7117: F, t100275: F, t100289: F, t18904: F, t18913: F, t18937: F, t18942: F, t19861: F, t20040: F, t25495: F, t27526: F, t27527: F, t27531: F, t53321: F, t6278: F, t93752: F, t93801: F) -> F {
    let t107140 = t7117 * t20020;
    let t107144 = t100275 + t100289 - t27526 * t27527 * t18942 / F::cast_from(144.0_f64) + t27526 * t27531 * t18937 / F::cast_from(216.0_f64) + t27526 * t27531 * t18913 / F::cast_from(108.0_f64) + F::cast_from(7.0_f64) / F::cast_from(648.0_f64) * t27526 * t53321 * t18904 - F::cast_from(0.57165357490759649296e-3_f64) * t93752 * t19861 - F::cast_from(0.57165357490759649296e-3_f64) * t93752 * t20040 - F::cast_from(0.95275595817932748827e-4_f64) * t93801 - F::cast_from(0.28582678745379824648e-3_f64) * t107140 + F::cast_from(0.22866142996303859718e-2_f64) * t25495 * t6278;
    t107144
}
