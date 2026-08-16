//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 676/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk676(t9379: f64, t9393: f64, t143: f64, t160: f64, t1901: f64, t28: f64, t446: f64, t89: f64, t9313: f64, t9318: f64, t9321: f64, t9324: f64, t9329: f64, t9333: f64, t9337: f64, t9340: f64, t9342: f64, t9345: f64, t9350: f64, t9355: f64, t9359: f64, t9363: f64) -> (f64, f64, f64) {
    let t9394 = t9379 + t9393;
    let t9396 = t143 * t9394 * t160;
    let t9400 = -2.0_f64 * t446 * t9313 + t446 * t9318 + 4.0_f64 / 9.0_f64 * t9321 - t446 * t9324 / 9.0_f64 - 10.0_f64 / 81.0_f64 * t446 * t9329 - t446 * t9333 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t446 * t9337 + 2.0_f64 / 3.0_f64 * t9340 + 2.0_f64 / 3.0_f64 * t9342 - 2.0_f64 / 3.0_f64 * t1901 * t9345 + t1901 * t9350 / 3.0_f64 + t1901 * t9355 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t1901 * t9359 - 2.0_f64 / 9.0_f64 * t1901 * t9363 + t89 * t28 * t9396 / 3.0_f64;
    (t9394, t9396, t9400)
}
