//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1136/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1136(t13313: f64, t2493: f64, t42164: f64, t42206: f64, t462: f64, t67329: f64, t67331: f64, t81010: f64, t81040: f64, t81042: f64, t88141: f64, t88145: f64, t88157: f64, t88176: f64, t88188: f64, t88219: f64, t88223: f64, t88612: f64, t9896: f64, t9916: f64) -> f64 {
    let t88983 = -8.0_f64 / 3.0_f64 * t81010 + 16.0_f64 / 3.0_f64 * t67329 - 8.0_f64 / 3.0_f64 * t67331 - 4.0_f64 * t462 * t9896 * t88223 + 40.0_f64 / 27.0_f64 * t462 * t42164 * t88141 + 4.0_f64 / 3.0_f64 * t462 * t2493 * t88219 + 4.0_f64 / 3.0_f64 * t462 * t2493 * t88145 + t42206 + 8.0_f64 * t81040 + 4.0_f64 / 3.0_f64 * t81042 + 4.0_f64 / 3.0_f64 * t462 * t9916 * t88188 - 4.0_f64 * t462 * t2493 * t88157 - 20.0_f64 / 9.0_f64 * t462 * t13313 * t88612 + 8.0_f64 * t462 * t2493 * t88176;
    t88983
}
