//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 869/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk869(t3787: f64, t562: f64, t3793: f64, t1338: f64, t1372: f64, t1352: f64, t1380: f64, t3851: f64, t3856: f64, t3879: f64, t553: f64, t1332: f64, t1336: f64, t1381: f64, t1383: f64, t3773: f64, t3777: f64, t544: f64, t564: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3897 = t3787 * t562;
    let t3898 = t3897 * t3793;
    let t3901 = t1338 * t1372;
    let t3902 = t3901 * t1352;
    let t3905 = t1380 * t3851;
    let t3907 = t1380 * t3856;
    let t3909 = t553 * t3879;
    let t3911 = 2.0_f64 * t1332 * t1383 + 2.0_f64 * t1336 * t3898 - 2.0_f64 * t1336 * t3902 - t1336 * t3905 - t1336 * t3907 - 2.0_f64 * t1381 * t3777 + t3773 * t564 + t3909 * t544;
    (t3898, t3901, t3902, t3905, t3907, t3909, t3911)
}
