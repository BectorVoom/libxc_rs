//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3406/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3406(t4707: f64, t3011: f64, t3014: f64, t981: f64, t11108: f64, t6396: f64, t2874: f64, t63657: f64, t935: f64, t19471: f64, t3022: f64, t15534: f64, t4719: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t63902 = t4707 * t4707;
    let t63906 = 0.34631718211362927518e2_f64 * t981 * t3011 * t63902 * t3014;
    let t63907 = t6396 * t11108;
    let t63916 = 4.0_f64 * t2874 * t63657 * t935;
    let t63918 = 0.69263436422725855036e2_f64 * t3022 * t19471;
    let t63920 = 0.11696447245269292414e1_f64 * t4719 * t15534;
    (t63902, t63906, t63907, t63916, t63918, t63920)
}
