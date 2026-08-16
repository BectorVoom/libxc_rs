//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 839/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk839(t22301: f64, t22329: f64, t845: f64, t91: f64, t1234: f64, t5337: f64, t10631: f64, t4191: f64, t5362: f64, t10797: f64, t14895: f64, t19246: f64, t19249: f64, t19278: f64, t19298: f64, t19301: f64, t19304: f64, t21981: f64, t22164: f64) -> (f64, f64, f64, f64, f64) {
    let t22330 = t22301 + t22329;
    let t22332 = t91 * t845 * t22330;
    let t22334 = t5337 * t1234;
    let t22336 = t91 * t10631 * t22334;
    let t22339 = t91 * t4191 * t5362;
    let t22345 = -6.0_f64 * t21981 - 4.0_f64 / 3.0_f64 * t14895 + t19246 - 2.0_f64 * t19249 - t22164 + t22332 / 2.0_f64 + 3.0_f64 / 8.0_f64 * t22336 - 3.0_f64 / 4.0_f64 * t22339 - 2.0_f64 / 3.0_f64 * t19278 - t10797 + t19298 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t19301 + 2.0_f64 / 9.0_f64 * t19304;
    (t22330, t22332, t22336, t22339, t22345)
}
