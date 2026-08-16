//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1352/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1352(t20800: f64, t5465: f64, t5480: f64, t3302: f64, t471: f64, t1214: f64, t20795: f64, t1287: f64, t21298: f64, t5464: f64, t21164: f64, t20900: f64, t487: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21465 = t20800 * t5465;
    let t21468 = t20800 * t5480;
    let t21471 = t3302 * t471;
    let t21472 = t21471 * t1214;
    let t21473 = t20795 * t21472;
    let t21480 = t21298 * t1287;
    let t21483 = t5464 * t1214;
    let t21484 = t20795 * t21483;
    let t21491 = t21164 * t1287;
    let t21495 = t487 * t20900 * t1287;
    (t21465, t21468, t21471, t21473, t21480, t21484, t21491, t21495)
}
