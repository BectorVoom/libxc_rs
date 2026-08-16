//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1308/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1308(t1749: f64, t303: f64, t3183: f64, t14413: f64, t2173: f64, t26857: f64, t5329: f64, t7709: f64, t8034: f64, t8042: f64, t93006: f64, t93008: f64, t93010: f64, t93012: f64, t96042: f64, t96045: f64, t96048: f64, t96052: f64) -> (f64, f64) {
    let t96061 = t303 * t1749 * t3183;
    let t96063 = 0.69505208333333333333e-3_f64 * t2173 * t5329 * t7709 * t14413 - 0.37069444444444444444e-2_f64 * t26857 * t8034 + 0.13265555555555555555e-1_f64 * t96042 - 0.13265555555555555555e-1_f64 * t96045 - 0.22109259259259259258e-2_f64 * t96048 + 0.33163888888888888888e-2_f64 * t96052 - 0.37069444444444444444e-2_f64 * t26857 * t8042 + 0.33163888888888888888e-2_f64 * t93006 + 0.88437037037037037034e-2_f64 * t93008 - 0.33163888888888888888e-2_f64 * t93010 + 0.22109259259259259258e-2_f64 * t93012 - 0.88437037037037037034e-2_f64 * t96061;
    (t96061, t96063)
}
