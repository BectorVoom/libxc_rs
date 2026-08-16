//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 966/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk966(t1102: f64, t3314: f64, t3692: f64, t11004: f64, t3579: f64, t3582: f64, t792: f64, t10997: f64, t3275: f64, t6967: f64, t795: f64, t3263: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11616 = t1102 * t3314 * t3692;
    let t11618 = t3579 * t11004;
    let t11619 = 5.0_f64 / 16.0_f64 * t11618;
    let t11621 = t3582 * t792;
    let t11622 = t10997 * t11621;
    let t11623 = t3275 * t11622;
    let t11624 = 45.0_f64 / 64.0_f64 * t11623;
    let t11625 = t6967 * t795;
    let t11626 = t3263 * t11625;
    (t11616, t11618, t11619, t11621, t11622, t11623, t11624, t11625, t11626)
}
