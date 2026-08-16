//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1096/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1096(t2650: f64, t546: f64, t565: f64, t10698: f64, t2559: f64, t10772: f64, t10810: f64, t2578: f64, t1577: f64, t2599: f64, t2096: f64, t2649: f64, t571: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39375 = t546 * t2650;
    let t39378 = t565 * t2650;
    let t39395 = t10698 * t2559;
    let t39400 = t10772 * t10810 * t2578;
    let t39403 = t1577 * t10810 * t2599;
    let t39409 = t571 * t2649 * t2096;
    (t39375, t39378, t39395, t39400, t39403, t39409)
}
