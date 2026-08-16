//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 659/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk659(t8796: f64, t8799: f64, t8802: f64, t8805: f64, t9010: f64, t9020: f64, t9035: f64, t9039: f64, t9043: f64, t9047: f64, t9052: f64, t9065: f64, t9068: f64) -> f64 {
    let t9162 = -2.0_f64 * t8805 - t9010 - 6.0_f64 * t9020 - 4.0_f64 / 3.0_f64 * t9065 + t9068 - 4.0_f64 / 9.0_f64 * t8796 + t8799 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t8802 + 2.0_f64 * t9035 - 2.0_f64 / 3.0_f64 * t9039 + t9043 + t9047 + 2.0_f64 / 3.0_f64 * t9052;
    t9162
}
