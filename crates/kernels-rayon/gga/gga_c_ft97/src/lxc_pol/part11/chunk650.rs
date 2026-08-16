//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 650/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk650(t1964: f64, t7765: f64, t356: f64, t89: f64, t569: f64, t7789: f64, t446: f64, t8796: f64, t8799: f64, t8802: f64, t8805: f64, t9010: f64, t9014: f64, t9020: f64, t9024: f64, t9028: f64) -> (f64, f64, f64, f64, f64) {
    let t9030 = t1964 * t7765;
    let t9032 = t89 * t356 * t9030;
    let t9034 = t569 * t7789;
    let t9035 = t446 * t9034;
    let t9037 = -2.0_f64 / 27.0_f64 * t8796 + t8799 / 18.0_f64 + t8802 / 27.0_f64 - t8805 / 3.0_f64 - t9010 / 6.0_f64 - t9014 / 18.0_f64 - t9020 + t9024 - 5.0_f64 / 81.0_f64 * t9028 - t9032 / 3.0_f64 + t9035 / 3.0_f64;
    (t9030, t9032, t9034, t9035, t9037)
}
