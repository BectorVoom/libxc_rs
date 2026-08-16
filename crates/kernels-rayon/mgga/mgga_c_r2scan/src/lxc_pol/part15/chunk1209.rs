//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1209/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1209(t11498: f64, t37282: f64, t3446: f64, t3453: f64, t7104: f64, t10655: f64, t11603: f64, t10922: f64, t11572: f64, t3308: f64, t3429: f64, t7136: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40509 = 3.0_f64 / 2.0_f64 * t37282 * t11498;
    let t40511 = t3446 * t3453 * t7104;
    let t40513 = t10655 * t11603;
    let t40515 = t10922 * t11603;
    let t40518 = t3429 * t3308 * t11572;
    let t40519 = 0.30487649791575028314e-3_f64 * t40518;
    let t40521 = t3446 * t3453 * t7136;
    (t40509, t40511, t40513, t40515, t40519, t40521)
}
