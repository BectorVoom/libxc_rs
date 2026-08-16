//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 293/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk293(t1163: f64, t1383: f64, t442: f64, t967: f64, t222: f64, t441: f64) -> (f64, f64, f64, f64, f64) {
    let t1384 = t1383 * t1163;
    let t1387 = t967 * t442;
    let t1388 = 0.5179538907796306876e-4_f64 * t1387;
    let t1389 = t441 * t222;
    let t1390 = 1.0_f64 / t1389;
    (t1384, t1387, t1388, t1389, t1390)
}
