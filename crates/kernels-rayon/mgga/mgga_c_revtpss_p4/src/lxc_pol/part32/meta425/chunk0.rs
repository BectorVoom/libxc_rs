//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1507/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1507(t1132: f64, t20337: f64, t1145: f64, t20318: f64, t141: f64, t20302: f64, t3417: f64, t20298: f64, t20310: f64, t20306: f64, t12327: f64, t6442: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20338 = t1132 * t20337;
    let t20340 = t1145 * t20318;
    let t20341 = t141 * t20340;
    let t20343 = t3417 * t20302;
    let t20344 = t141 * t20343;
    let t20346 = t3417 * t20298;
    let t20347 = t141 * t20346;
    let t20349 = t1145 * t20310;
    let t20350 = t141 * t20349;
    let t20352 = t1145 * t20306;
    let t20353 = t141 * t20352;
    let t20356 = t12327 * t6442;
    (t20338, t20341, t20344, t20347, t20350, t20353, t20356)
}
