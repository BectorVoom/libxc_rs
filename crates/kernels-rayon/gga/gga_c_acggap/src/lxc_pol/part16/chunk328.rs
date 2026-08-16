//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 328/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk328(t336: f64, t429: f64, t513: f64, t1140: f64, t515: f64, t1137: f64, t500: f64, t1050: f64, t1063: f64, t1124: f64, t1126: f64, t1130: f64, t1474: f64, t1477: f64, t1481: f64, t1484: f64) -> (f64, f64, f64, f64) {
    let t1511 = t336 * t429 * t513;
    let t1514 = t1140 * t515;
    let t1516 = t1137 * t500;
    let t1524 = t1124 + 0.489e0_f64 * t1050 - t1126 + 0.489e0_f64 * t1474 + 0.7335e0_f64 * t1477 - 0.61125e-1_f64 * t1481 - 0.36675e0_f64 * t1484 - 0.61125e-1_f64 * t1063 + t1130;
    (t1511, t1514, t1516, t1524)
}
