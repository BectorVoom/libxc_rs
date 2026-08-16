//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1161/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1161(t204: f64, t376: f64, t370: f64, t374: f64, t9697: f64, t10473: f64, t361: f64, t363: f64, t42342: f64, t42345: f64, t3131: f64, t221: f64, t339: f64, t42813: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43216 = t204 * t376;
    let t43253 = 7.0_f64 / 31104.0_f64 * t370 * t374 * t9697 * t376;
    let t43288 = 1.0_f64 / t10473 / t361;
    let t43291 = t42342 * t43288 * t363 * t42345;
    let t43292 = t3131 * t3131;
    let t43307 = 5.0_f64 / 486.0_f64 * t339 * t221 * t42813;
    (t43216, t43253, t43288, t43291, t43292, t43307)
}
