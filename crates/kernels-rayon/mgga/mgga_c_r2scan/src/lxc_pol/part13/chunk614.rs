//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 614/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk614(t322: f64, t1074: f64, t829: f64, t1300: f64, t327: f64, t3370: f64, t3373: f64, t834: f64, t330: f64, t1079: f64, t837: f64, t3369: f64) -> (f64, f64, f64, f64) {
    let t332 = 0.25e1_f64 < t322;
    let t3376 = t1074 * t829;
    let t3381 = -0.64e0_f64 * t3370 * t327 - 0.128e1_f64 * t3373 * t829 - 0.128e1_f64 * t1300 * t3376 - 0.64e0_f64 * t834 * t3370;
    let t3382 = t3381 * t330;
    let t3383 = t1079 * t837;
    let t3384 = t3383 * t330;
    let t3386 = piecewise3(t332, 0.0_f64, t3369);
    (t3381, t3382, t3384, t3386)
}
