//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1384/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1384(t28423: f64, t8144: f64, t102767: f64, t102769: f64, t103582: f64, t1598: f64, t2239: f64, t28403: f64, t28544: f64, t60299: f64, t7898: f64, t94669: f64, t98934: f64, t98938: f64, t98942: f64, t98945: f64, t98946: f64) -> f64 {
    let t103768 = t8144 * t28423;
    let t103779 = -0.69505208333333333333e-3_f64 * t60299 * t1598 * t2239 - 0.4946917361111111111e-3_f64 * t28544 * t28403 + 0.46336805555555555557e-3_f64 * t103768 - 0.55273148148148148147e-3_f64 * t94669 - 0.41224311342592592593e-4_f64 * t98934 + 0.33163888888888888888e-2_f64 * t102767 + 0.61836467013888888889e-4_f64 * t98938 + 0.11054629629629629629e-2_f64 * t102769 + 0.29479012345679012345e-2_f64 * t98942 + 0.92754700520833333333e-4_f64 * t7898 * t103582 + t98945 - 0.58958024691358024689e-2_f64 * t98946;
    t103779
}
