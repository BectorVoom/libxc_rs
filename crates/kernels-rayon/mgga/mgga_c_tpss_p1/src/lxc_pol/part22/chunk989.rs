//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 989/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk989(t3621: f64, t750: f64, t762: f64, t1368: f64, t2133: f64, t2158: f64, t339: f64, t790: f64, t3632: f64, t10623: f64, t10630: f64, t10632: f64, t10635: f64, t10638: f64, t10642: f64, t2147: f64, t761: f64, t797: f64, t8127: f64, t8131: f64, t8133: f64, t8168: f64, t8171: f64) -> f64 {
    let t10644 = t762 * t3621 * t750;
    let t10648 = t762 * t1368 * t2133;
    let t10652 = t339 * t2158 * t790;
    let t10654 = 7.0_f64 / 1152.0_f64 * t10652 * t3632;
    let t10656 = -t797 * t10623 / 768.0_f64 - 35.0_f64 / 1152.0_f64 * t8127 - 119.0_f64 / 1728.0_f64 * t8131 + 7.0_f64 / 1152.0_f64 * t8133 + t10630 - t761 * t10632 / 48.0_f64 - 35.0_f64 / 216.0_f64 * t10635 - t8171 * t10638 / 4.0_f64 - t10642 + t2147 * t10644 / 8.0_f64 + t2147 * t10648 / 16.0_f64 - t10654 - 7.0_f64 / 48.0_f64 * t8168;
    t10656
}
