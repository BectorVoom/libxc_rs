//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1342/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1342(t26777: f64, t26790: f64, t415: f64, t26252: f64, t26258: f64, t26278: f64, t26280: f64, t26284: f64, t26289: f64, t26293: f64, t26296: f64, t26300: f64, t26304: f64, t26306: f64) -> (f64, f64) {
    let t26792 = (t26777 + t26790) * t415;
    let t26805 = 0.25367901234567901233e-1_f64 * t26252 + 0.2283111111111111111e0_f64 * t26258 - 0.11415555555555555555e0_f64 * t26278 + 0.13698666666666666667e0_f64 * t26280 - 0.41095999999999999999e0_f64 * t26284 + 0.41095999999999999998e0_f64 * t26289 - 0.34246666666666666665e-1_f64 * t26293 + 0.41096e0_f64 * t26296 - 0.61644e0_f64 * t26300 + 0.10274e0_f64 * t26304 - 0.13698666666666666667e0_f64 * t26306;
    (t26792, t26805)
}
