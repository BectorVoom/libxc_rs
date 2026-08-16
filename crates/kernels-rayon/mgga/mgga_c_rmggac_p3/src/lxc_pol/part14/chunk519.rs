//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 519/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk519(t5679: f64, t5734: f64, t1163: f64, t1166: f64, t1168: f64, t1174: f64, t1452: f64, t1454: f64, t1455: f64, t1459: f64, t228: f64, t4435: f64, t4438: f64, t4444: f64, t458: f64, t462: f64, t5531: f64, t5533: f64, t5538: f64, t5540: f64, t5543: f64, t5555: f64, t5558: f64, t5561: f64, t5564: f64, t5567: f64, t598: f64) -> (f64, f64) {
    let t5735 = t5679 + t5734;
    let t5738 = t5531 * t228 + 2.0_f64 * t5533 * t1455 + t1452 * t1168 / 2.0_f64 + t5538 * t1455 + t5540 * t1455 + t1454 * t5543 / 2.0_f64 - 5.0_f64 / 16.0_f64 * t598 * t4435 + t598 * t4438 / 4.0_f64 + t1163 * t1459 / 4.0_f64 + t1166 * t1459 / 4.0_f64 - 5.0_f64 / 8.0_f64 * t458 * t5555 + t458 * t5558 / 2.0_f64 + 45.0_f64 / 64.0_f64 * t4444 * t5561 - 5.0_f64 / 8.0_f64 * t1174 * t5564 - 5.0_f64 / 16.0_f64 * t1174 * t5567 + t462 * t5735 / 4.0_f64;
    (t5735, t5738)
}
