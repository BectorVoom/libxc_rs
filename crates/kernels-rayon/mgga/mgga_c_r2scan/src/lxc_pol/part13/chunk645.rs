//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 645/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk645(t1060: f64, t3613: f64, t783: f64, t3283: f64, t3286: f64, t3289: f64, t3318: f64, t3323: f64, t3346: f64, t3586: f64, t3589: f64, t3592: f64, t3595: f64, t3598: f64, t3600: f64, t3604: f64, t3608: f64, t3611: f64) -> f64 {
    let t3615 = t783 * t3613 * t1060;
    let t3617 = -t3283 + t3286 - t3289 - 0.54878743191129263322e-1_f64 * t3586 - 0.27439371595564631661e-1_f64 * t3589 - 0.43341108700271342816e-1_f64 * t3592 - 0.13002332610081402845e0_f64 * t3595 - 0.43341108700271342816e-1_f64 * t3598 + 0.43341108700271342816e-1_f64 * t3600 - t3318 + t3323 + 0.21831846657716620896e-2_f64 * t3604 + 0.65495539973149862688e-2_f64 * t3608 + 0.21831846657716620896e-2_f64 * t3611 - 0.21831846657716620896e-2_f64 * t3615 - t3346;
    t3617
}
