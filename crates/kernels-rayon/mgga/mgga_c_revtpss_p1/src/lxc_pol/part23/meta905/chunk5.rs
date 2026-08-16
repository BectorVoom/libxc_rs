//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2911/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2911(t52037: f64, t52346: f64, t63338: f64, t63340: f64, t63342: f64, t63361: f64, t63371: f64, t63447: f64, t63453: f64, t63459: f64, t63464: f64, t77559: f64, t77561: f64, t77566: f64, t77570: f64, t77575: f64, t77581: f64, t77586: f64, t77590: f64, t77594: f64) -> f64 {
    let t77596 = -0.37083333333333333333e-1_f64 * t63338 + 0.12361111111111111111e-1_f64 * t63340 + 0.10300925925925925926e-1_f64 * t63342 + 0.55625000000000000001e-1_f64 * t63361 - 0.37083333333333333334e-1_f64 * t63371 + t52346 - 0.82407407407407407407e-2_f64 * t52037 + 0.92708333333333333334e-2_f64 * t63447 - 0.82407407407407407408e-2_f64 * t63453 + 0.24722222222222222223e-1_f64 * t63459 + 0.61805555555555555553e-2_f64 * t77559 - 0.18541666666666666667e-1_f64 * t77561 + 0.12361111111111111111e0_f64 * t77566 - 0.30902777777777777778e-1_f64 * t77570 - 0.27469135802469135803e-1_f64 * t77575 - 0.12361111111111111111e-1_f64 * t63464 + 0.18541666666666666667e-1_f64 * t77581 - 0.61805555555555555555e-2_f64 * t77586 - 0.22249999999999999999e0_f64 * t77590 + 0.11125e0_f64 * t77594;
    t77596
}
