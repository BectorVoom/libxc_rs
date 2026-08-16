//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 586/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk586(t7624: f64, t7666: f64, t515: f64, t235: f64, t665: f64, t848: f64, t884: f64, t1243: f64, t128: f64, t118: f64, t2001: f64, t675: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7667 = t7624 + t7666;
    let t7668 = t515 * t7667;
    let t7669 = t235 * t7668;
    let t7670 = 0.19957069503106347607e-1_f64 * t7669;
    let t7672 = t665 * t848;
    let t7673 = t884 * t7672;
    let t7674 = 0.59871208509319042821e-1_f64 * t7673;
    let t7675 = t128 * t1243;
    let t7676 = t118 * t7675;
    let t7677 = t2001 * t7676;
    let t7678 = t675 * t7677;
    (t7667, t7668, t7670, t7672, t7674, t7677, t7678)
}
