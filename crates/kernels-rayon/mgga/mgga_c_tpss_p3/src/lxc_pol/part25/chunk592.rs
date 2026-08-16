//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 592/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk592(t1219: f64, t1253: f64, t507: f64, t541: f64, t1953: f64, t1957: f64, t1960: f64, t1964: f64, t1967: f64, t1973: f64, t1286: f64, t577: f64) -> (f64, f64, f64, f64) {
    let t3374 = t1219 * t1253;
    let t3391 = t507 * t541;
    let t3416 = -t1953 + t1957 - t1960 + t1964 - t1967 + t1973;
    let t3418 = t1286 * t577;
    (t3374, t3391, t3416, t3418)
}
