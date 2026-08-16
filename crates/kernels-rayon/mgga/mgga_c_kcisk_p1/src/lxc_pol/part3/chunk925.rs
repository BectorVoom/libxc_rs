//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 925/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk925(t13642: f64, t13673: f64, t1254: f64, t1232: f64, t4079: f64, t346: f64, t360: f64, t4082: f64, t13589: f64, t13522: f64, t13526: f64, t13530: f64, t13533: f64, t13536: f64, t13540: f64, t13543: f64, t13546: f64, t13549: f64, t13552: f64, t13555: f64) -> (f64, f64, f64, f64) {
    let t13674 = t13642 + t13673;
    let t13675 = t13674 * t1254;
    let t13679 = 1.0_f64 / t4079 / t1232;
    let t13680 = t346 * t13679;
    let t13682 = 1.0_f64 / t4082 / t360;
    let t13683 = t13589 * t13682;
    let t13686 = 0.28842592592592592592e-1_f64 * t13522;
    let t13697 = -t13686 - 0.12361111111111111111e-1_f64 * t13526 + 0.61805555555555555556e-2_f64 * t13530 - 0.18541666666666666667e-1_f64 * t13533 + 0.92708333333333333334e-2_f64 * t13536 - 0.10300925925925925926e-1_f64 * t13540 + 0.37083333333333333333e-1_f64 * t13543 - 0.18541666666666666666e-1_f64 * t13546 - 0.55625000000000000001e-1_f64 * t13549 + 0.55625000000000000001e-1_f64 * t13552 - 0.92708333333333333333e-2_f64 * t13555;
    (t13675, t13680, t13683, t13697)
}
