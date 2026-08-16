//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1041/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1041(t41454: f64, t92: f64, t9568: f64, t41448: f64, t9570: f64, t2404: f64, t41464: f64, t41693: f64, t41696: f64, t41700: f64, t41703: f64, t41705: f64, t41707: f64, t41709: f64, t41713: f64) -> (f64, f64, f64, f64, f64) {
    let t41716 = t92 * t9568 * t41454;
    let t41718 = t9570 * t41448;
    let t41720 = t92 * t2404 * t41718;
    let t41723 = t92 * t2404 * t41464;
    let t41725 = 8.0_f64 * t41693 - 12.0_f64 * t41696 + 2.0_f64 * t41700 + 8.0_f64 / 3.0_f64 * t41703 + 112.0_f64 / 81.0_f64 * t41705 - 8.0_f64 / 9.0_f64 * t41707 - 16.0_f64 / 27.0_f64 * t41709 + 40.0_f64 / 9.0_f64 * t41713 - 20.0_f64 / 9.0_f64 * t41716 - 8.0_f64 * t41720 + 8.0_f64 * t41723;
    (t41716, t41718, t41720, t41723, t41725)
}
