//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 680/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk680(t1882: f64, t2187: f64, t2202: f64, t161: f64, t7943: f64, t89: f64, t1901: f64, t446: f64, t9402: f64, t9405: f64, t9408: f64, t9412: f64, t9416: f64, t9420: f64, t9425: f64, t9430: f64, t9434: f64, t9442: f64, t9446: f64, t9449: f64) -> f64 {
    let t9451 = t1882 * t2187;
    let t9453 = t1882 * t2202;
    let t9457 = 28.0_f64 / 81.0_f64 * t89 * t7943 * t161;
    let t9458 = 2.0_f64 * t446 * t9402 + 2.0_f64 / 9.0_f64 * t9405 - t446 * t9408 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t9412 + 4.0_f64 / 9.0_f64 * t446 * t9416 + 2.0_f64 / 3.0_f64 * t1901 * t9420 + t1901 * t9425 / 3.0_f64 - t446 * t9430 - 2.0_f64 * t446 * t9434 - 2.0_f64 * t446 * t9442 + 2.0_f64 * t446 * t9446 - 2.0_f64 / 3.0_f64 * t9449 - 2.0_f64 / 3.0_f64 * t9451 + t9453 / 9.0_f64 - t9457;
    t9458
}
