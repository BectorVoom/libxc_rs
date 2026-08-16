//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 847/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk847(t32114: f64, t32332: f64, t32349: f64, t34413: f64, t34418: f64, t34485: f64, t34489: f64, t34493: f64, t34497: f64, t34501: f64, t34505: f64, t34509: f64) -> f64 {
    let t34534 = t32114 + t34413 / 18.0_f64 + t34418 / 3.0_f64 - t34485 / 6.0_f64 - t32332 - 2.0_f64 / 9.0_f64 * t34489 - 2.0_f64 * t34493 + 4.0_f64 / 3.0_f64 * t34497 + t32349 + t34501 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t34505 - t34509 / 3.0_f64;
    t34534
}
