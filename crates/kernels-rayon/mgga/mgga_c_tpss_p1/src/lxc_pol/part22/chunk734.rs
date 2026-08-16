//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 734/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk734(t1388: f64, t768: f64, t1379: f64, t220: f64, t229: f64, t2415: f64, t339: f64, t3630: f64, t3665: f64, t3692: f64, t3703: f64, t3704: f64, t3713: f64, t783: f64, t813: f64) -> (f64, f64) {
    let t3716 = t768 * t1388;
    let t3721 = -t1379 * t2415 * t339 + t220 * t229 * t3692 - t339 * t3665 * t813 - t339 * t3716 * t783 + 2.0_f64 * t3630 * t3703 * t3704 - t3704 * t3713 * t783;
    (t3716, t3721)
}
