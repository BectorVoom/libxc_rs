//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1205/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1205(t30213: f64, t3332: f64, t7628: f64, t12543: f64, t22731: f64, t27996: f64, t6165: f64, t28000: f64, t22868: f64, t30292: f64, t26185: f64, t30296: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43379 = t7628 * t3332 * t30213;
    let t43381 = t22731 * t12543;
    let t43384 = t6165 * t3332 * t27996;
    let t43387 = t6165 * t3332 * t28000;
    let t43390 = t22868 * t3332 * t30292;
    let t43393 = t26185 * t3332 * t30296;
    (t43379, t43381, t43384, t43387, t43390, t43393)
}
