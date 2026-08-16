//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 329/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk329(t1378: f64, t1938: f64, t286: f64, t1367: f64, t1368: f64, t1930: f64, t1934: f64, t493: f64, t500: f64) -> (f64, f64, f64) {
    let t1939 = t1378 * t1938;
    let t1940 = t286 * t1939;
    let t1943 = -t1930 * t500 / 36.0_f64 + t1367 + t1368 * t1934 / 288.0_f64 - t493 * t1940 / 96.0_f64;
    (t1939, t1940, t1943)
}
