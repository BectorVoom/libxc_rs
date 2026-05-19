//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1384/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1384<F: Float>(t28423: F, t8144: F, t102767: F, t102769: F, t103582: F, t1598: F, t2239: F, t28403: F, t28544: F, t60299: F, t7898: F, t94669: F, t98934: F, t98938: F, t98942: F, t98945: F, t98946: F) -> F {
    let t103768 = t8144 * t28423;
    let t103779 = -F::cast_from(0.69505208333333333333e-3_f64) * t60299 * t1598 * t2239 - F::cast_from(0.4946917361111111111e-3_f64) * t28544 * t28403 + F::cast_from(0.46336805555555555557e-3_f64) * t103768 - F::cast_from(0.55273148148148148147e-3_f64) * t94669 - F::cast_from(0.41224311342592592593e-4_f64) * t98934 + F::cast_from(0.33163888888888888888e-2_f64) * t102767 + F::cast_from(0.61836467013888888889e-4_f64) * t98938 + F::cast_from(0.11054629629629629629e-2_f64) * t102769 + F::cast_from(0.29479012345679012345e-2_f64) * t98942 + F::cast_from(0.92754700520833333333e-4_f64) * t7898 * t103582 + t98945 - F::cast_from(0.58958024691358024689e-2_f64) * t98946;
    t103779
}
