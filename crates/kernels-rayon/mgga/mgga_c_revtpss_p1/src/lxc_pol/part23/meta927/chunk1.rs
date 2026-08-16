//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3010/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3010(t53252: f64, t53253: f64, t63338: f64, t63340: f64, t63342: f64, t63361: f64, t63371: f64, t63447: f64, t63453: f64, t63459: f64, t63464: f64, t77559: f64, t77561: f64, t77566: f64, t77570: f64, t77575: f64, t77581: f64, t77586: f64, t77590: f64, t77594: f64) -> f64 {
    let t80027 = -0.59266666666666666668e-1_f64 * t63338 + 0.19755555555555555556e-1_f64 * t63340 + 0.16462962962962962963e-1_f64 * t63342 + 0.88900000000000000002e-1_f64 * t63361 - 0.59266666666666666668e-1_f64 * t63371 + t53252 - t53253 + 0.14816666666666666667e-1_f64 * t63447 - 0.13170370370370370371e-1_f64 * t63453 + 0.39511111111111111112e-1_f64 * t63459 + 0.9877777777777777778e-2_f64 * t77559 - 0.29633333333333333334e-1_f64 * t77561 + 0.19755555555555555556e0_f64 * t77566 - 0.49388888888888888889e-1_f64 * t77570 - 0.43901234567901234568e-1_f64 * t77575 - 0.19755555555555555556e-1_f64 * t63464 + 0.29633333333333333334e-1_f64 * t77581 - 0.9877777777777777778e-2_f64 * t77586 - 0.35560000000000000001e0_f64 * t77590 + 0.1778e0_f64 * t77594;
    t80027
}
