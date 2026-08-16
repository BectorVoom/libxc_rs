//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 925/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk925(t11409: f64, t11746: f64, t16046: f64, t16050: f64, t16052: f64, t16523: f64, t21268: f64, t21273: f64, t21275: f64, t21278: f64, t21581: f64, t11727: f64, t11730: f64, t1319: f64, t1410: f64, t16500: f64, t16503: f64, t1897: f64, t21267: f64, t21537: f64, t21542: f64, t21551: f64, t21558: f64, t3821: f64, t3824: f64, t456: f64, t5481: f64, t5503: f64, t5510: f64, t6957: f64, t6964: f64) -> f64 {
    let t21582 = 0.14865e-1_f64 * t21273 - 0.1982e-1_f64 * t21275 - 0.991e-2_f64 * t21278 + 0.1982e-1_f64 * t21268 - t11746 - 0.18344444444444444444e-2_f64 * t11409 - 0.36688888888888888888e-2_f64 * t16046 + t16523 - 0.55033333333333333332e-2_f64 * t16050 - 0.55033333333333333332e-2_f64 * t16052 + t21581;
    let t21585 = 3.0_f64 / 16.0_f64 * t11727 * t21537 - t11730 * t6957 / 8.0_f64 - t3821 * t21542 / 4.0_f64 - t16500 * t5503 / 4.0_f64 + t16503 * t1897 / 2.0_f64 + t5510 * t5481 / 2.0_f64 - t3821 * t21551 / 8.0_f64 + t3824 * t6964 / 4.0_f64 + t1410 * t21267 / 4.0_f64 + t21558 * t1319 / 4.0_f64 + t456 * t21582 / 2.0_f64;
    t21585
}
