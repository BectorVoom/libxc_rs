//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1736/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1736(t29314: f64, t29375: f64, t533: f64, t1390: f64, t26905: f64, t7687: f64, t19451: f64, t1983: f64, t2036: f64, t2040: f64, t2079: f64, t22574: f64, t28002: f64, t28030: f64, t29211: f64, t29214: f64, t29219: f64, t29222: f64, t29241: f64, t29243: f64, t29247: f64, t29252: f64, t4028: f64, t574: f64, t6287: f64, t6468: f64, t652: f64, t7458: f64, t7685: f64, t7796: f64, t7802: f64, t7904: f64, t7943: f64) -> (f64, f64, f64, f64, f64) {
    let t29376 = t29314 + t29375;
    let t29377 = t533 * t29376;
    let t29378 = t29377 * t1390;
    let t29380 = t26905 * t7687;
    let t29394 = -2.0_f64 * t19451 * t2040 - t1983 * t29222 + 2.0_f64 * t1983 * t29243 + 6.0_f64 * t1983 * t29252 + t1983 * t29378 + 6.0_f64 * t1983 * t29380 - t2036 * t6287 - 4.0_f64 * t2040 * t28002 - 2.0_f64 * t2040 * t28030 + t2079 * t6468 - 6.0_f64 * t22574 * t29247 - 2.0_f64 * t29211 * t652 - 2.0_f64 * t29214 * t652 - 4.0_f64 * t29219 * t652 + t29241 * t574 - 4.0_f64 * t4028 * t7796 - 4.0_f64 * t4028 * t7802 - 4.0_f64 * t7458 * t7796 + 6.0_f64 * t7685 * t7904 - 2.0_f64 * t7685 * t7943;
    (t29376, t29377, t29378, t29380, t29394)
}
