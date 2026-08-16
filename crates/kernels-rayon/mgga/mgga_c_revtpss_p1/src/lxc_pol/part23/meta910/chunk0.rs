//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2923/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2923(t52091: f64, t52092: f64, t63338: f64, t63340: f64, t63342: f64, t63361: f64, t63371: f64, t63447: f64, t63453: f64, t63459: f64, t63464: f64, t77559: f64, t77561: f64, t77566: f64, t77570: f64, t77575: f64, t77581: f64, t77586: f64, t77590: f64, t77594: f64) -> f64 {
    let t77797 = -4.0_f64 / 3.0_f64 * t63338 + 4.0_f64 / 9.0_f64 * t63340 + 10.0_f64 / 27.0_f64 * t63342 + 2.0_f64 * t63361 - 4.0_f64 / 3.0_f64 * t63371 + t52091 - t52092 + t63447 / 3.0_f64 - 8.0_f64 / 27.0_f64 * t63453 + 8.0_f64 / 9.0_f64 * t63459 + 2.0_f64 / 9.0_f64 * t77559 - 2.0_f64 / 3.0_f64 * t77561 + 40.0_f64 / 9.0_f64 * t77566 - 10.0_f64 / 9.0_f64 * t77570 - 80.0_f64 / 81.0_f64 * t77575 - 4.0_f64 / 9.0_f64 * t63464 + 2.0_f64 / 3.0_f64 * t77581 - 2.0_f64 / 9.0_f64 * t77586 - 8.0_f64 * t77590 + 4.0_f64 * t77594;
    t77797
}
