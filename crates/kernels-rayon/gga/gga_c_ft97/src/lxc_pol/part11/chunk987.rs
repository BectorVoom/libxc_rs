//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 987/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk987(t39723: f64, t39708: f64, t39711: f64, t39715: f64, t39717: f64, t39721: f64, t39728: f64, t39732: f64, t39737: f64, t39741: f64, t39744: f64, t39747: f64, t39753: f64, t39757: f64) -> f64 {
    let t40546 = 8.0_f64 / 27.0_f64 * t39723;
    let t40555 = -2.0_f64 * t39708 + 4.0_f64 / 3.0_f64 * t39711 - 2.0_f64 / 3.0_f64 * t39715 - 2.0_f64 / 9.0_f64 * t39717 + 4.0_f64 / 3.0_f64 * t39721 + t40546 + 20.0_f64 / 81.0_f64 * t39728 + t39732 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t39737 + 2.0_f64 / 9.0_f64 * t39741 + 4.0_f64 / 9.0_f64 * t39744 - 4.0_f64 / 27.0_f64 * t39747 + 4.0_f64 / 3.0_f64 * t39753 - 2.0_f64 / 3.0_f64 * t39757;
    t40555
}
