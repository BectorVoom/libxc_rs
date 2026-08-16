//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 861/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk861(t1901: f64, t28: f64, t32589: f64, t34696: f64, t34700: f64, t34703: f64, t34707: f64, t34710: f64, t34714: f64, t34718: f64, t34722: f64, t34726: f64, t34729: f64, t34732: f64, t446: f64, t89: f64) -> f64 {
    let t34735 = t89 * t28 * t34696 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t34700 + 4.0_f64 / 3.0_f64 * t446 * t34703 + t1901 * t34707 / 9.0_f64 - t32589 - t446 * t34710 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t34714 + 2.0_f64 / 3.0_f64 * t446 * t34718 - 2.0_f64 / 3.0_f64 * t446 * t34722 - 4.0_f64 / 3.0_f64 * t1901 * t34726 - t446 * t34729 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t34732;
    t34735
}
