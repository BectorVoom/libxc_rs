//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 442/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk442(t7124: f64, t871: f64, t296: f64, t1901: f64, t193: f64, t446: f64, t6272: f64, t6298: f64, t6359: f64, t7033: f64, t7038: f64, t7042: f64, t7047: f64, t7051: f64, t7055: f64, t7059: f64, t7093: f64, t7098: f64, t7102: f64, t7107: f64, t7111: f64, t7116: f64, t89: f64) -> (f64, f64) {
    let t7125 = t871 * t7124;
    let t7126 = t296 * t7125;
    let t7129 = t6272 + t1901 * t7033 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t7038 - t446 * t7042 / 3.0_f64 + t446 * t7047 / 3.0_f64 - t446 * t7051 / 3.0_f64 - t6298 - t446 * t7055 / 9.0_f64 - t446 * t7059 / 3.0_f64 + t89 * t193 * t7093 / 3.0_f64 - t446 * t7098 / 3.0_f64 + t6359 + t1901 * t7102 / 9.0_f64 + t446 * t7107 / 3.0_f64 - t446 * t7111 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t7116 - t446 * t7126 / 3.0_f64;
    (t7126, t7129)
}
