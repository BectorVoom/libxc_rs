//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1137/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1137(t13682: f64, t13683: f64, t2493: f64, t3910: f64, t3917: f64, t42110: f64, t42124: f64, t462: f64, t53287: f64, t81048: f64, t81050: f64, t81057: f64, t88149: f64, t88153: f64, t88161: f64, t88165: f64, t88169: f64, t88180: f64, t88184: f64, t88227: f64, t88277: f64, t88606: f64, t9896: f64, t9916: f64) -> f64 {
    let t89018 = -12.0_f64 * t462 * t3917 * t88606 + 8.0_f64 * t462 * t3910 * t88184 + 8.0_f64 * t462 * t9896 * t88169 + 8.0_f64 * t462 * t42124 * t88165 + 8.0_f64 / 3.0_f64 * t462 * t3917 * t88149 - 8.0_f64 / 9.0_f64 * t462 * t3910 * t88153 + 8.0_f64 / 3.0_f64 * t13682 * t13683 * t88277 - 8.0_f64 / 3.0_f64 * t462 * t42110 * t88227 - 16.0_f64 / 3.0_f64 * t462 * t9916 * t88161 + 2.0_f64 * t462 * t2493 * t88180 + 8.0_f64 / 3.0_f64 * t81048 - 16.0_f64 / 9.0_f64 * t81050 + 40.0_f64 / 81.0_f64 * t81057 + 112.0_f64 / 81.0_f64 * t53287;
    t89018
}
