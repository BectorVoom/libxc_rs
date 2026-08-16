//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1107/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1107(t3275: f64, t3276: f64, t39286: f64, t10943: f64, t11603: f64, t10918: f64, t3579: f64, t495: f64, t797: f64, t10615: f64, t11559: f64, t2333: f64, t2847: f64) -> (f64, f64, f64, f64, f64) {
    let t39289 = 5.0_f64 / 16.0_f64 * t3275 * t3276 * t39286;
    let t39290 = t10943 * t11603;
    let t39295 = t3579 * t495 * t10918 * t797 / 2.0_f64;
    let t39298 = 5.0_f64 / 8.0_f64 * t3275 * t10615 * t11559;
    let t39299 = t2333 * t2847;
    (t39289, t39290, t39295, t39298, t39299)
}
