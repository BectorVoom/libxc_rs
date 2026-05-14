//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 950/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk950<F: Float>(t13293: F, t3200: F, t1804: F, t2845: F, t3210: F, t4554: F, t13238: F, t13243: F, t13247: F, t13252: F, t13254: F, t13258: F, t13263: F, t13268: F, t13271: F, t13275: F, t13278: F, t13282: F, t13286: F, t13290: F, t9524: F, t9529: F, t9536: F, t9552: F) -> (F, F, F) {
    let t13294 = t3200 * t13293;
    let t13296 = t1804 * t2845;
    let t13297 = t3210 * t13296;
    let t13298 = t4554 * t13297;
    let t13300 = -0.33163888888888888888e-2 * t9524 - 0.88437037037037037034e-2 * t13238 + t13243 + 0.11054629629629629629e-2 * t9529 - 0.3684876543209876543e-3 * t9536 + 0.88437037037037037034e-2 * t13247 + 0.44218518518518518517e-2 * t13252 - 0.33163888888888888888e-2 * t13254 - 0.33163888888888888888e-2 * t13258 + 0.55273148148148148147e-3 * t13263 + 0.33163888888888888888e-2 * t13268 + t13271 - 0.73697530864197530861e-3 * t9552 + 0.13265555555555555555e-1 * t13275 - t13278 - 0.16581944444444444444e-2 * t13282 - 0.16581944444444444444e-2 * t13286 - 0.27636574074074074073e-2 * t13290 + 0.11054629629629629629e-2 * t13294 + 0.18424382716049382715e-2 * t13298;
    (t13294, t13298, t13300)
}
