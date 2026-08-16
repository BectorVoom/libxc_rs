//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 817/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk817(t1901: f64, t33008: f64, t33009: f64, t33012: f64, t33016: f64, t33017: f64, t33020: f64, t33024: f64, t33028: f64, t33031: f64, t33036: f64, t33041: f64, t33046: f64, t446: f64) -> f64 {
    let t33049 = t33008 - t446 * t33009 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t33012 + t33016 - 2.0_f64 / 3.0_f64 * t446 * t33017 - t446 * t33020 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t33024 - t446 * t33028 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t33031 + t1901 * t33036 / 9.0_f64 + t1901 * t33041 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t33046;
    t33049
}
