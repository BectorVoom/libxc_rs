//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 892/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk892(t609: f64, t7490: f64, t1608: f64, t286: f64, t1599: f64, t2100: f64, t2106: f64, t4424: f64, t4439: f64, t6138: f64, t6141: f64, t6149: f64, t6169: f64, t619: f64, t7403: f64, t7414: f64, t7418: f64, t7422: f64, t7426: f64, t7431: f64) -> (f64, f64, f64) {
    let t614 = 0.0_f64 < t609;
    let t7492 = piecewise3(t614, t7490, -t7490);
    let t7493 = t1608 * t7492;
    let t7494 = t286 * t7493;
    let t7497 = 11.0_f64 / 216.0_f64 * t7403 * t619 - t6138 / 108.0_f64 - t6141 * t2100 / 108.0_f64 + t6141 * t2106 / 36.0_f64 - t4424 + t6149 / 864.0_f64 - t6169 / 288.0_f64 + t1599 * t7414 / 432.0_f64 - t4439 * t7418 / 288.0_f64 - t1599 * t7422 / 288.0_f64 + t1599 * t7426 / 576.0_f64 + t1599 * t7431 / 96.0_f64 - t1599 * t7494 / 192.0_f64;
    (t7492, t7493, t7497)
}
