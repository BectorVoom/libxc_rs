//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 291/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk291(t1180: f64, t31: f64, t217: f64, t673: f64, t1184: f64, t605: f64, t476: f64, t221: f64, t1190: f64, t608: f64, t209: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1465 = t1180 * t31;
    let t1466 = t673 * t217;
    let t1467 = t1465 * t1466;
    let t1468 = t605 * t1184;
    let t1469 = t1468 * t476;
    let t1470 = t221 * t1469;
    let t1473 = t1190 * t608;
    let t1475 = t605 * t209;
    (t1465, t1466, t1467, t1468, t1469, t1470, t1473, t1475)
}
