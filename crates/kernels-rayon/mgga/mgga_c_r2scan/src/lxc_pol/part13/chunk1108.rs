//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1108/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1108(t39299: f64, t795: f64, t3275: f64, t3276: f64, t10615: f64, t11555: f64, t1053: f64, t1102: f64, t1103: f64, t7028: f64, t2850: f64, t4176: f64) -> (f64, f64, f64, f64) {
    let t39300 = t39299 * t795;
    let t39303 = 5.0_f64 / 8.0_f64 * t3275 * t3276 * t39300;
    let t39306 = 5.0_f64 / 8.0_f64 * t3275 * t10615 * t11555;
    let t39309 = t1102 * t1053 * t1103 * t7028;
    let t39311 = t4176 * t2850;
    (t39303, t39306, t39309, t39311)
}
