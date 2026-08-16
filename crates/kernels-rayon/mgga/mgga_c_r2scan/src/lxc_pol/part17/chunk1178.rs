//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1178/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1178(t28000: f64, t3332: f64, t6165: f64, t22868: f64, t30292: f64, t26185: f64, t30296: f64, t29779: f64, t7614: f64, t29270: f64, t10856: f64, t9273: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43387 = t6165 * t3332 * t28000;
    let t43390 = t22868 * t3332 * t30292;
    let t43393 = t26185 * t3332 * t30296;
    let t43396 = t7614 * t3332 * t29779;
    let t43399 = t6165 * t3332 * t29270;
    let t43401 = t10856 * t9273;
    (t43387, t43390, t43393, t43396, t43399, t43401)
}
