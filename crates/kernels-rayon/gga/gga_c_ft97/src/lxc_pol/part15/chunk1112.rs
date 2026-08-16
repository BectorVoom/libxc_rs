//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1112/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1112(t42087: f64, t88252: f64, t9920: f64, t2497: f64, t88239: f64, t42095: f64, t9953: f64, t1131: f64, t21655: f64, t13688: f64, t13689: f64, t13693: f64, t18293: f64, t21457: f64, t2372: f64, t2486: f64, t42094: f64, t462: f64, t5053: f64, t67078: f64, t67097: f64, t737: f64, t80963: f64, t88240: f64, t88248: f64, t9707: f64, t9952: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t88253 = t42087 * t88252;
    let t88257 = t9920 * t88252;
    let t88261 = t2497 * t88239;
    let t88269 = t42095 * t88252;
    let t88273 = t9953 * t88252;
    let t88277 = t21655 * t1131;
    let t88286 = -2.0_f64 / 3.0_f64 * t462 * t2486 * t88240 - 36.0_f64 * t462 * t9707 * t18293 * t5053 - t462 * t737 * t88248 / 3.0_f64 + 40.0_f64 / 9.0_f64 * t462 * t9952 * t88253 + 8.0_f64 * t462 * t737 * t88257 + 2.0_f64 * t462 * t737 * t88261 + 8.0_f64 * t462 * t2372 * t80963 * t1131 - 80.0_f64 / 81.0_f64 * t462 * t42094 * t88269 - 8.0_f64 * t462 * t2486 * t88273 - 8.0_f64 * t13688 * t13689 * t88277 - 8.0_f64 * t13688 * t13693 * t21457 + 16.0_f64 / 9.0_f64 * t67078 - 16.0_f64 / 27.0_f64 * t67097;
    (t88253, t88257, t88261, t88269, t88273, t88277, t88286)
}
