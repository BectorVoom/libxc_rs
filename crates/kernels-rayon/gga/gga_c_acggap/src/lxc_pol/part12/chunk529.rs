//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 529/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk529(t3237: f64, t431: f64, t438: f64, t314: f64, t847: f64, t150: f64, t383: f64, t390: f64, t174: f64, t943: f64, t6: f64, t965: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3238 = t3237 * t431;
    let t3240 = t3237 * t438;
    let t3242 = t847 * t314;
    let t3243 = t3242 * t150;
    let t3244 = t3243 * t383;
    let t3246 = 0.64311027177104605458e-3_f64 * t3244 * t390;
    let t3253 = t174 * t943;
    let t3266 = t6 * t965;
    (t3238, t3240, t3242, t3243, t3246, t3253, t3266)
}
