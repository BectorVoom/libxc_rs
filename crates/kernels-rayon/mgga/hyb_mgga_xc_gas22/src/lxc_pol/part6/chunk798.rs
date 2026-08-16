//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 798/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk798(t312: f64, t4368: f64, t4371: f64, t4374: f64, t1454: f64, t1455: f64, t1459: f64, t1479: f64, t1480: f64, t1486: f64, t3951: f64, t3957: f64, t3963: f64, t3969: f64, t3974: f64, t398: f64, t401: f64, t414: f64, t415: f64, t423: f64, t431: f64, t4356: f64, t4363: f64, t4383: f64, t4389: f64, t4390: f64, t4394: f64, t4397: f64, t4400: f64) -> f64 {
    let t4403 = t312 * t4368;
    let t4404 = t4371 * t4374;
    let t4407 = 0.28999131295963805491e1_f64 * t431 * t414 * t4356 * t423 - 0.2854310864347144482e1_f64 * t431 * t1479 * t4363 * t1486 + 0.70082276486377300979e0_f64 * t431 * t4368 * t4371 * t4374 + 0.458714896073149408e1_f64 * t398 * t3951 * t401 - 40.0_f64 / 9.0_f64 * t1459 * t3969 + 0.28999131295963805491e1_f64 * t415 * t4383 - 0.52822214337494074078e1_f64 * t1454 * t3957 * t1455 + 0.14685052460713464727e1_f64 * t4389 * t3963 * t4390 + 50.0_f64 / 9.0_f64 * t4394 * t3974 + 50.0_f64 / 9.0_f64 * t4397 * t3974 - 0.2854310864347144482e1_f64 * t1480 * t4400 + 0.70082276486377300979e0_f64 * t4403 * t4404;
    t4407
}
