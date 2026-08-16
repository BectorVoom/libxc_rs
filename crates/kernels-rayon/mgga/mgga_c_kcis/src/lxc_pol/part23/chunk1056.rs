//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1056/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1056(t1548: f64, t3734: f64, t1552: f64, t3738: f64, t4123: f64, t4288: f64, t582: f64, sigma2: f64) -> (f64, f64, f64, f64) {
    let t27537 = t3734 * t1548;
    let t27539 = t3738 * t1552;
    let t27541 = t4123 * t4288;
    let t27543 = sigma2 * t582;
    (t27537, t27539, t27541, t27543)
}
