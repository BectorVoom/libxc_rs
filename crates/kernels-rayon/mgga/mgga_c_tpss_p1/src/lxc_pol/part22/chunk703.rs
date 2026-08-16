//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 703/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk703(t116: f64, t2061: f64, t117: f64, t2105: f64, t1279: f64, t1281: f64, t3403: f64, t547: f64, t548: f64, t1953: f64, t1957: f64, t1960: f64, t1964: f64, t1967: f64, t1973: f64) -> (f64, f64, f64, f64) {
    let t3407 = t116 * t2061;
    let t3410 = t117 * t2105;
    let t3413 = 6.0_f64 * t1279 * t1281 + t3403 * t548 + 6.0_f64 * t3407 * t547 + 3.0_f64 * t3410 * t547;
    let t3416 = -t1953 + t1957 - t1960 + t1964 - t1967 + t1973;
    (t3407, t3410, t3413, t3416)
}
