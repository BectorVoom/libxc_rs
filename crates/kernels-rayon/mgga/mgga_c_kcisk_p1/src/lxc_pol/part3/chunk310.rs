//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 310/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk310(t442: f64, t451: f64, t1056: f64, t1471: f64, t1402: f64, t416: f64, t140: f64, t1429: f64, t1434: f64, t1460: f64, t1469: f64, t1470: f64, t460: f64, t476: f64, t479: f64) -> (f64, f64, f64, f64) {
    let t1472 = t451 * t442;
    let t1474 = t1471 * t1472 * t1056;
    let t1477 = t416 * t1402;
    let t1481 = 0.619125e-2_f64 * t1460 * t460 + 0.9286875e-2_f64 * t476 * t1429 - 0.619125e-2_f64 * t476 * t1434 - t1469 - 0.26531111111111111111e-1_f64 * t1470 * t1474 - 0.39796666666666666666e-1_f64 * t140 * t479 * t1477;
    (t1472, t1474, t1477, t1481)
}
