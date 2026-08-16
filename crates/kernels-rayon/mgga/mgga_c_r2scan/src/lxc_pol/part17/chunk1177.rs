//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1177/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1177(t3190: f64, t3319: f64, t3320: f64, t5103: f64, t22790: f64, t30057: f64, t3332: f64, t30213: f64, t7628: f64, t12543: f64, t22731: f64, t27996: f64, t6165: f64) -> (f64, f64, f64, f64, f64) {
    let t43372 = t5103 * t3319 * t3320 * t3190;
    let t43376 = t22790 * t3332 * t30057;
    let t43379 = t7628 * t3332 * t30213;
    let t43381 = t22731 * t12543;
    let t43384 = t6165 * t3332 * t27996;
    (t43372, t43376, t43379, t43381, t43384)
}
