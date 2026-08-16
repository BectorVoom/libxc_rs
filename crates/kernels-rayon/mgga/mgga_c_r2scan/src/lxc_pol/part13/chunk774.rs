//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 774/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk774(t2169: f64, t2219: f64, t1543: f64, t1632: f64, t551: f64, t2196: f64, t481: f64, t6343: f64, t566: f64, t560: f64, t549: f64, t110: f64, t6238: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6496 = t2169 * t2219;
    let t6499 = t1632 * t1543;
    let t6500 = t551 * t6499;
    let t6501 = t2196 * t6500;
    let t6503 = t6343 * t481;
    let t6504 = t551 * t6503;
    let t6505 = t566 * t6504;
    let t6507 = t6343 * t560;
    let t6508 = t551 * t6507;
    let t6509 = t549 * t6508;
    let t6511 = t6238 * t110;
    (t6496, t6499, t6501, t6503, t6505, t6507, t6509, t6511)
}
