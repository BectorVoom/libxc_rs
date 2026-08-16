//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 753/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk753(t481: f64, t6212: f64, t6211: f64, t6480: f64, t2168: f64, t2195: f64, t6343: f64, t551: f64, t566: f64, t560: f64, t549: f64, t110: f64, t6238: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6481 = t6212 * t481;
    let t6482 = t6211 * t6481;
    let t6483 = t6480 * t6482;
    let t6493 = t2195 * t2168;
    let t6503 = t6343 * t481;
    let t6504 = t551 * t6503;
    let t6505 = t566 * t6504;
    let t6507 = t6343 * t560;
    let t6508 = t551 * t6507;
    let t6509 = t549 * t6508;
    let t6511 = t6238 * t110;
    (t6481, t6483, t6493, t6503, t6505, t6507, t6509, t6511)
}
