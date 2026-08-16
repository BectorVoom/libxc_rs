//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 776/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk776(t10504: f64, t2881: f64, t10440: f64, t10444: f64, t10448: f64, t10453: f64, t10458: f64, t10461: f64, t10463: f64, t10467: f64, t10471: f64, t10475: f64, t10482: f64, t10488: f64, t10495: f64, t10500: f64, t1901: f64) -> (f64, f64) {
    let t10505 = t2881 * t10504;
    let t10508 = t1901 * t10440 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t1901 * t10444 + 2.0_f64 / 3.0_f64 * t1901 * t10448 - 2.0_f64 / 3.0_f64 * t1901 * t10453 + t1901 * t10458 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t10461 - 2.0_f64 / 9.0_f64 * t10463 + t1901 * t10467 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t1901 * t10471 - 2.0_f64 / 9.0_f64 * t1901 * t10475 + 2.0_f64 / 9.0_f64 * t1901 * t10482 + 2.0_f64 / 9.0_f64 * t1901 * t10488 - 2.0_f64 / 3.0_f64 * t1901 * t10495 + t1901 * t10500 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t1901 * t10505;
    (t10505, t10508)
}
