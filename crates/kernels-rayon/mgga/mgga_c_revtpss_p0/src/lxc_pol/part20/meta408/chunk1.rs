//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1511/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1511(t1014: f64, t11150: f64, t1003: f64, t11735: f64, t221: f64, t345: f64, t346: f64, t624: f64, t1050: f64, t41: f64, t1011: f64, t1012: f64, t1017: f64, t11767: f64, t3236: f64, t3241: f64, t344: f64, t348: f64, t39443: f64, t39449: f64, t42716: f64, t42719: f64, t42721: f64, t42724: f64, t42727: f64, sigma0: f64) -> (f64, f64) {
    let t42731 = t1014 * t11150;
    let t42740 = t1003 * t11735;
    let t42745 = 5.0_f64 / 486.0_f64 * t345 * t221 * t624 * t346;
    let t42747 = 1.0_f64 / t41 / t1050;
    let t42748 = sigma0 * t42747;
    let t42752 = 5.0_f64 / 972.0_f64 * t42716 + t42719 / 108.0_f64 - 154.0_f64 / 243.0_f64 * t42721 * t1017 + 11.0_f64 / 81.0_f64 * t42724 + t42727 / 36.0_f64 - 2.0_f64 / 9.0_f64 * t3241 * t11767 - t1011 * t1012 * t42731 * t39443 / 12.0_f64 - t1011 * t1012 * t3236 * t39449 / 48.0_f64 - 10.0_f64 / 243.0_f64 * t42740 - t42745 + 1309.0_f64 / 486.0_f64 * t42748 * t344 * t348;
    (t42748, t42752)
}
