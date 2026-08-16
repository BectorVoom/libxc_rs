//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 495/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk495(t2224: f64, t529: f64, t119: f64, t1266: f64, t122: f64, t507: f64, t1234: f64, t506: f64, t2168: f64, t546: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2225 = t529 * t2224;
    let t2228 = t1266 * t119;
    let t2231 = 0.16463622957338778997e-1_f64 * t2228 * t122 * t507;
    let t2232 = t506 * t1234;
    let t2233 = t529 * t2232;
    let t2236 = t546 * t2168;
    (t2225, t2228, t2231, t2232, t2233, t2236)
}
