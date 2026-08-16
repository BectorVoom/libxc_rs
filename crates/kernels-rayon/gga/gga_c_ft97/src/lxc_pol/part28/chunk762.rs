//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 762/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk762(t32355: f64, t432: f64, t28: f64, t89: f64, t32325: f64, t370: f64, t27: f64, t32114: f64, t32118: f64, t32123: f64, t32328: f64, t32332: f64, t32336: f64, t32341: f64, t32345: f64, t32349: f64, t32353: f64) -> (f64, f64, f64, f64, f64) {
    let t32356 = t32355 * t432;
    let t32357 = t28 * t32356;
    let t32358 = t89 * t32357;
    let t32360 = t370 * t32325;
    let t32362 = t89 * t27 * t32360;
    let t32364 = t32114 + t32118 / 18.0_f64 + t32123 / 3.0_f64 - t32328 / 6.0_f64 - t32332 - 2.0_f64 / 9.0_f64 * t32336 - 2.0_f64 * t32341 + 4.0_f64 / 3.0_f64 * t32345 + t32349 + t32353 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t32358 - t32362 / 3.0_f64;
    (t32356, t32358, t32360, t32362, t32364)
}
