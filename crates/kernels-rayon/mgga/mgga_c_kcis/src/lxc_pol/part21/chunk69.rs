//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 69/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk69(t157: f64, t161: f64, t155: f64, rho0: f64, rho1: f64) -> (f64, f64, f64, f64) {
    let t162 = t157 * t161;
    let t164 = 1.0_f64 + t155 / 8.0_f64 - t162 / 64.0_f64;
    let t165 = 1.0_f64 / t164;
    let t167 = rho0 - rho1;
    (t162, t164, t165, t167)
}
