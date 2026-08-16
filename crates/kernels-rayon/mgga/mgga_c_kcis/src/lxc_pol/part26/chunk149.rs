//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 149/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk149(t209: f64, t613: f64, t617: f64, t612: f64) -> (f64, f64, f64, f64) {
    let t619 = t209 * t613 * t617;
    let t622 = 1.0_f64 + t612 * t619 / 192.0_f64;
    let t623 = f64::ln(t622);
    let t625 = 1.0_f64 + 0.66725e-1_f64 * t623;
    let t626 = 1.0_f64 / t625;
    (t619, t622, t625, t626)
}
