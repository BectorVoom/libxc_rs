//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1004/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1004(t11280: f64, t1286: f64, t137404: f64, t137412: f64, t137415: f64, t137418: f64, t137442: f64, t144505: f64, t144511: f64, t144520: f64, t144524: f64, t144538: f64, t1526: f64, t1527: f64, t22883: f64, t25582: f64, t25846: f64, t25942: f64, t25960: f64, t26027: f64, t28: f64, t32026: f64, t32038: f64, t3266: f64, t342: f64, t343: f64, t34592: f64, t34596: f64, t34601: f64, t356: f64, t379: f64, t461: f64, t5495: f64, t6455: f64, t7150: f64, t7151: f64, t7152: f64, t72: f64) -> f64 {
    let t144551 = -t144505 / 36.0_f64 - t34592 * t32038 / 6.0_f64 - t137404 / 9.0_f64 - t144511 / 12.0_f64 - t25582 * t7150 * t7152 / 6.0_f64 - t7151 * t461 * t26027 / 6.0_f64 + t144520 / 18.0_f64 + t5495 * t34596 / 18.0_f64 + t144524 / 18.0_f64 - t32026 * t34601 / 6.0_f64 - t1526 * t1527 * t25942 / 12.0_f64 - t1526 * t11280 * t25960 / 6.0_f64 - t1286 * t28 * t22883 * t3266 - t144538 / 54.0_f64 + t1286 * t356 * t6455 * t379 / 18.0_f64 + t137412 / 18.0_f64 + t137415 / 18.0_f64 - t137418 / 36.0_f64 - t137442 - t342 * t343 * t72 * t25846 / 4.0_f64;
    t144551
}
