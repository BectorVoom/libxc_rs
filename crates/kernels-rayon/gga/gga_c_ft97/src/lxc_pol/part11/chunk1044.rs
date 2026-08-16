//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1044/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1044(t39370: f64, t668: f64, t683: f64, t92: f64, t41728: f64, t41731: f64, t41733: f64, t41735: f64, t41737: f64, t41739: f64, t41741: f64, t41745: f64, t41746: f64, t41748: f64, t41755: f64) -> (f64, f64, f64) {
    let t41757 = t668 * t39370;
    let t41759 = t92 * t683 * t41757;
    let t41761 = -2.0_f64 / 3.0_f64 * t41728 - 8.0_f64 / 9.0_f64 * t41731 + 16.0_f64 / 9.0_f64 * t41733 - 16.0_f64 / 9.0_f64 * t41735 + 8.0_f64 / 9.0_f64 * t41737 + 8.0_f64 / 3.0_f64 * t41739 - 8.0_f64 / 3.0_f64 * t41741 + t41745 + 40.0_f64 / 81.0_f64 * t41746 + 4.0_f64 / 9.0_f64 * t41748 - 80.0_f64 / 81.0_f64 * t41755 - t41759 / 3.0_f64;
    (t41757, t41759, t41761)
}
