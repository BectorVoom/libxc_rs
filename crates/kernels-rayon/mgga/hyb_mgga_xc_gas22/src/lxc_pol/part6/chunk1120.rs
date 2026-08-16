//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1120/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1120(t1410: f64, t3513: f64, t2478: f64, t4273: f64, t967: f64, t7075: f64, t4270: f64, t2523: f64, t4269: f64, t2521: f64, t3517: f64, t4243: f64, t7150: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11022 = t1410 * t3513;
    let t11024 = 4.0_f64 * t2478 * t11022;
    let t11025 = t4273 * t967;
    let t11027 = 0.96491876992155210402e2_f64 * t7075 * t11025;
    let t11028 = t4270 * t967;
    let t11030 = 2.0_f64 * t2478 * t11028;
    let t11031 = t4269 * t2523;
    let t11032 = t11031 * t967;
    let t11034 = 0.16081979498692535067e2_f64 * t2521 * t11032;
    let t11035 = t3517 * t3513;
    let t11037 = 0.32163958997385070134e2_f64 * t2521 * t11035;
    let t11038 = t4243 * t7150;
    (t11022, t11024, t11025, t11027, t11028, t11030, t11031, t11032, t11034, t11035, t11037, t11038)
}
