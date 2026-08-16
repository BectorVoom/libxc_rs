//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1956/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1956(t22081: f64, t26028: f64, t22085: f64, t22048: f64, t27940: f64, t22089: f64, t22146: f64, t26004: f64, t6884: f64, t6850: f64, t94513: f64, t22041: f64, t7252: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t108526 = t26028 * t22081;
    let t108528 = t26028 * t22085;
    let t108531 = t27940 * t22048;
    let t108533 = t27940 * t22089;
    let t108535 = t27940 * t22146;
    let t108537 = t26004 * t6884;
    let t108539 = t94513 * t6850;
    let t108541 = t7252 * t22041;
    (t108526, t108528, t108531, t108533, t108535, t108537, t108539, t108541)
}
