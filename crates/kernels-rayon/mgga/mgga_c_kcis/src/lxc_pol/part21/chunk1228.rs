//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1228/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1228(t10525: f64, t283: f64, t2865: f64, t374: f64, t1165: f64, t982: f64, t1169: f64, t3473: f64, t3463: f64, t3329: f64, t7738: f64, t3668: f64, t7807: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t92522 = t10525 * t283;
    let t92525 = t374 * t2865;
    let t92532 = t1165 * t982;
    let t92537 = t1169 * t2865;
    let t92540 = t3473 * t982;
    let t92544 = t3463 * t982;
    let t92564 = t7738 * t3329;
    let t92576 = t7807 * t3668;
    (t92522, t92525, t92532, t92537, t92540, t92544, t92564, t92576)
}
