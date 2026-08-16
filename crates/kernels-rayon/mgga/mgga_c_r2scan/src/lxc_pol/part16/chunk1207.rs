//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1207/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1207(t11736: f64, t11744: f64, t10760: f64, t20298: f64, t30628: f64, t20305: f64, t29837: f64, t11640: f64, t30370: f64, t11842: f64, t2651: f64, t10810: f64, t574: f64, t9292: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43407 = t11744 * t11736;
    let t43410 = t20298 * t10760 * t30628;
    let t43413 = t20305 * t10760 * t29837;
    let t43415 = t30370 * t11640;
    let t43418 = t2651 * t11842;
    let t43421 = t574 * t10810 * t9292;
    (t43407, t43410, t43413, t43415, t43418, t43421)
}
