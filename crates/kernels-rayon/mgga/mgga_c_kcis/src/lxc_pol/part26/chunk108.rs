//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 108/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk108(t286: f64, t499: f64, t493: f64) -> (f64, f64, f64, f64) {
    let t500 = t286 * t499;
    let t503 = 1.0_f64 + t493 * t500 / 96.0_f64;
    let t504 = f64::ln(t503);
    let t506 = 1.0_f64 + 0.66725e-1_f64 * t504;
    let t507 = 1.0_f64 / t506;
    (t500, t503, t506, t507)
}
