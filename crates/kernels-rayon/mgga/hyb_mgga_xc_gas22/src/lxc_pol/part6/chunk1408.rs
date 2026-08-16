//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1408/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1408(t22050: f64, t22054: f64, t22058: f64, t22061: f64, t22064: f64, t22068: f64, t22072: f64, t22076: f64, t22080: f64, t22084: f64, t22089: f64, t22090: f64, t22094: f64, t22095: f64, t26007: f64, t26010: f64, t26012: f64) -> f64 {
    let t30451 = t22050 + t22054 + t22058 + 48.0_f64 * t22061 - 0.69263436422725855034e2_f64 * t26007 + 2.0_f64 * t22064 + t22068 - 240.0_f64 * t26010 - 0.11393789434848516923e-2_f64 * t26012 - t22072 + t22076 + 192.0_f64 * t22080 + 96.0_f64 * t22084 - t22089 - 0.70178683471615754484e1_f64 * t22090 + t22094 - 160.0_f64 * t22095;
    t30451
}
