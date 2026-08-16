//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 768/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk768(t32113: f64, t32331: f64, t32348: f64, t32118: f64, t32123: f64, t32328: f64, t32336: f64, t32341: f64, t32345: f64, t32353: f64, t32358: f64, t32362: f64) -> (f64, f64, f64, f64) {
    let t32446 = t32113 / 6.0_f64;
    let t32449 = 2.0_f64 / 3.0_f64 * t32331;
    let t32453 = t32348 / 3.0_f64;
    let t32456 = t32446 + t32118 / 6.0_f64 + t32123 - t32328 / 2.0_f64 - t32449 - 2.0_f64 / 3.0_f64 * t32336 - 6.0_f64 * t32341 + 4.0_f64 * t32345 + t32453 + t32353 / 3.0_f64 + 2.0_f64 * t32358 - t32362;
    (t32446, t32449, t32453, t32456)
}
