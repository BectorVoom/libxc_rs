//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 523/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk523(t3237: f64, t431: f64, t438: f64, t314: f64, t847: f64, t150: f64, t383: f64, t390: f64, t336: f64, t360: f64, t1016: f64, t141: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3238 = t3237 * t431;
    let t3240 = t3237 * t438;
    let t3242 = t847 * t314;
    let t3243 = t3242 * t150;
    let t3244 = t3243 * t383;
    let t3246 = 0.64311027177104605458e-3_f64 * t3244 * t390;
    let t3282 = t336 * t360;
    let t3300 = t141 * t1016;
    (t3238, t3240, t3242, t3243, t3246, t3282, t3300)
}
