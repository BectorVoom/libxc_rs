//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1371/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1371(t1774: f64, t3568: f64, t247: f64, t3719: f64, t15687: f64, t3623: f64, t3782: f64, t1263: f64, t1794: f64, t372: f64, t12712: f64, t3629: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17345 = t1774 * t3568;
    let t17347 = t247 * t3719 * t17345;
    let t17350 = t3623 * t15687;
    let t17351 = t3782 * t17350;
    let t17352 = t1263 * t1794;
    let t17353 = t372 * t17352;
    let t17354 = t12712 * t3629;
    (t17345, t17347, t17350, t17351, t17353, t17354)
}
