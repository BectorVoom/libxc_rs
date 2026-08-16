//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1191/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1191(t10997: f64, t3275: f64, t40289: f64, t3446: f64, t3453: f64, t7133: f64, t795: f64, t983: f64, t481: f64, t37327: f64, t4176: f64, t11487: f64, t37282: f64) -> (f64, f64, f64, f64, f64) {
    let t40292 = 45.0_f64 / 64.0_f64 * t3275 * t10997 * t40289;
    let t40294 = t3446 * t3453 * t7133;
    let t40296 = t983 * t795;
    let t40297 = t40296 * t481;
    let t40300 = 15.0_f64 / 8.0_f64 * t37327 * t4176 * t40297;
    let t40302 = 15.0_f64 / 8.0_f64 * t37282 * t11487;
    (t40292, t40294, t40296, t40300, t40302)
}
