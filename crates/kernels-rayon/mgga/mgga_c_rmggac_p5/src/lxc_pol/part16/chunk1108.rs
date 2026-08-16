//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1108/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1108(t1364: f64, t1635: f64, t2211: f64, t2471: f64, t36508: f64, t36515: f64, t37964: f64, t41637: f64, t41641: f64, t41647: f64, t41648: f64, t45622: f64, t47378: f64, t47381: f64, t47385: f64, t47390: f64, t47393: f64, t5898: f64, t6421: f64, t6441: f64, t699: f64, t884: f64, t903: f64, t9530: f64) -> f64 {
    let t49032 = 0.5987120850931904282e-1_f64 * t47378 - 0.8980681276397856423e-1_f64 * t47381 - 0.7273243107798757795e0_f64 * t41637 + 0.4363945864679254677e0_f64 * t41641 + 0.2993560425465952141e-1_f64 * t47385 + t37964 - 0.23948483403727617128e0_f64 * t1364 * t699 * t6421 - 0.11974241701863808564e0_f64 * t884 * t2211 * t45622 - 0.47896966807455234256e0_f64 * t1364 * t2471 * t1635 - 0.23948483403727617128e0_f64 * t884 * t9530 * t5898 + 0.17961362552795712846e0_f64 * t903 * t699 * t6441 - 0.95793933614910468512e0_f64 * t47390 - 0.31931311204970156171e0_f64 * t47393 - 0.66211599834018861287e-4_f64 * t36508 + t41647 - t41648 - 0.66211599834018861287e-4_f64 * t36515;
    t49032
}
