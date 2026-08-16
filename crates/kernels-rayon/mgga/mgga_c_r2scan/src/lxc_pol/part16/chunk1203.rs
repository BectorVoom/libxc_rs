//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1203/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1203(t10710: f64, t30691: f64, t37582: f64, t10708: f64, t27977: f64, t10810: f64, t1592: f64, t9380: f64, t3190: f64, t3319: f64, t3320: f64, t5103: f64) -> (f64, f64, f64, f64) {
    let t43362 = t37582 * t10710 * t30691;
    let t43365 = t10708 * t10710 * t27977;
    let t43368 = t1592 * t10810 * t9380;
    let t43372 = t5103 * t3319 * t3320 * t3190;
    (t43362, t43365, t43368, t43372)
}
