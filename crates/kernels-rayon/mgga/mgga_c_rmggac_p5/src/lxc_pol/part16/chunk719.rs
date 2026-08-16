//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 719/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk719(t10414: f64, t10480: f64, t82: f64, t72: f64, t10458: f64, t515: f64, t235: f64, t10067: f64, t10073: f64, t10079: f64, t10417: f64, t884: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10481 = t10414 + t10480;
    let t10482 = t82 * t10481;
    let t10483 = t72 * t10482;
    let t10484 = t515 * t10458;
    let t10485 = t235 * t10484;
    let t10486 = 0.19957069503106347607e-1_f64 * t10485;
    let t10488 = 0.212822999466489197e-4_f64 * t10067;
    let t10490 = 0.1702583995731913576e-4_f64 * t10073;
    let t10491 = 0.5107751987195740728e-4_f64 * t10079;
    let t10492 = t884 * t10417;
    (t10481, t10482, t10483, t10484, t10486, t10488, t10490, t10491, t10492)
}
