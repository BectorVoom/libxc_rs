//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1230/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1230(t17351: f64, t17455: f64, t20705: f64, t20787: f64, t25633: f64, t25636: f64, t30284: f64, t30287: f64, t665: f64, t672: f64, t3532: f64, t667: f64) -> (f64, f64, f64) {
    let t30288 = t17455 - 28.0_f64 / 27.0_f64 * t17351 - 28.0_f64 / 9.0_f64 * t20705 + t20787 + 4.0_f64 / 3.0_f64 * t25633 - t25636 - t30284 / 3.0_f64 + t30287;
    let t30289 = t665 * t30288;
    let t30291 = t672 * t30288;
    let t30293 = t3532 * t667;
    (t30289, t30291, t30293)
}
