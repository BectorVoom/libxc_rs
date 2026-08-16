//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 376/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk376(t2354: f64, t2355: f64, t680: f64, t2318: f64, t2321: f64, t2323: f64, t2327: f64, t2329: f64, t2331: f64) -> (f64, f64) {
    let t2357 = t2354 * t2355 * t680;
    let t2366 = -0.57538888888888888889e0_f64 * t2318 + 0.11507777777777777778e1_f64 * t2321 + 0.40256666666666666667e0_f64 * t2323 + 0.366775e-1_f64 * t2327 + 0.73355e-1_f64 * t2329 + 0.137975e0_f64 * t2331;
    (t2357, t2366)
}
