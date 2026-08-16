//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1236/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1236(t14160: f64, t40574: f64, t43744: f64, t11531: f64, t11629: f64, t3275: f64, t11498: f64, t40282: f64, t11502: f64, t40664: f64, t11556: f64, t40713: f64) -> (f64, f64, f64, f64, f64) {
    let t43747 = 5.0_f64 / 4.0_f64 * t40574 * t14160 * t43744;
    let t43750 = 5.0_f64 / 8.0_f64 * t3275 * t11629 * t11531;
    let t43752 = 3.0_f64 / 2.0_f64 * t40282 * t11498;
    let t43754 = 3.0_f64 / 2.0_f64 * t40664 * t11502;
    let t43756 = 5.0_f64 / 8.0_f64 * t40713 * t11556;
    (t43747, t43750, t43752, t43754, t43756)
}
