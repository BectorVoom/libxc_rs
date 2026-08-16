//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1155/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1155(t29296: f64, t29335: f64, t29377: f64, t29410: f64, t589: f64, t2069: f64, t28558: f64, t27494: f64, t7271: f64, t7397: f64, t7940: f64, t22300: f64, t2253: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29412 = t29296 + t29335 + t29377 + t29410;
    let t29413 = t29412 * t589;
    let t29415 = 2.0_f64 * t28558 * t2069;
    let t29417 = 2.0_f64 * t27494 * t7271;
    let t29418 = t7940 * t7397;
    let t29419 = t22300 * t2253;
    (t29412, t29413, t29415, t29417, t29418, t29419)
}
