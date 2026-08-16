//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 807/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk807(t34111: f64, t34115: f64, t34120: f64, t34124: f64, t34126: f64, t34130: f64, t34133: f64, t34136: f64, t34139: f64, t34142: f64, t34146: f64, t34150: f64, t446: f64) -> f64 {
    let t34153 = 2.0_f64 / 3.0_f64 * t446 * t34111 + 4.0_f64 / 3.0_f64 * t446 * t34115 + 2.0_f64 / 3.0_f64 * t446 * t34120 + t34124 - 2.0_f64 / 3.0_f64 * t446 * t34126 + 2.0_f64 / 3.0_f64 * t446 * t34130 - t446 * t34133 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t34136 - 2.0_f64 / 3.0_f64 * t446 * t34139 - t446 * t34142 / 3.0_f64 - t446 * t34146 / 3.0_f64 - t446 * t34150 / 3.0_f64;
    t34153
}
