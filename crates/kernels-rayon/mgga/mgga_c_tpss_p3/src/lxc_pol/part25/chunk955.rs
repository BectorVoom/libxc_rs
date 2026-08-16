//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 955/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk955(t1111: f64, t12445: f64, t1571: f64, t3087: f64, t11453: f64, t4252: f64, t3080: f64, t1569: f64, t453: f64, t1141: f64, t2738: f64, t4270: f64, t9561: f64) -> (f64, f64, f64, f64, f64) {
    let t12446 = t1111 * t12445;
    let t12448 = t1571 * t3087;
    let t12463 = t11453 * t4252;
    let t12465 = t3080 * t12463 / 2304.0_f64;
    let t12470 = t453 * t1569;
    let t12472 = t1141 * t12470 * t2738;
    let t12475 = t9561 * t4270;
    (t12446, t12448, t12465, t12472, t12475)
}
