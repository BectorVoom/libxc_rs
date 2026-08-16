//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 704/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk704(t27185: f64, t28: f64, t89: f64, t23931: f64, t27145: f64, t27150: f64, t27155: f64, t27161: f64, t27163: f64, t27168: f64, t27171: f64, t27176: f64, t27179: f64, t27183: f64) -> (f64, f64, f64) {
    let t27186 = t28 * t27185;
    let t27187 = t89 * t27186;
    let t27189 = t27145 / 3.0_f64 - t27150 / 2.0_f64 - t27155 / 2.0_f64 - 3.0_f64 / 8.0_f64 * t27161 - t27163 / 18.0_f64 + t27168 / 6.0_f64 + t27171 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t23931 + 2.0_f64 * t27176 - 2.0_f64 / 3.0_f64 * t27179 + 2.0_f64 * t27183 + 2.0_f64 * t27187;
    (t27186, t27187, t27189)
}
