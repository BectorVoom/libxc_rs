//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3212/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3212(t10355: f64, t10368: f64, t13312: f64, t13325: f64, t13328: f64, t1480: f64, t18281: f64, t21732: f64, t21733: f64, t21736: f64, t21741: f64, t21742: f64, t21761: f64, t2251: f64, t2258: f64, t2275: f64, t2282: f64, t4201: f64, t4210: f64, t44: f64, t46065: f64, t46074: f64, t56: f64, t5819: f64, t5825: f64, t606: f64, t614: f64) -> f64 {
    let t60987 = 5.0_f64 / 162.0_f64 * t56 * t46074 * t5819 * t2251 + 5.0_f64 / 9.0_f64 * t56 * t4210 * t13312 + 5.0_f64 / 9.0_f64 * t56 * t2282 * t18281 * t606 + 5.0_f64 / 18.0_f64 * t56 * t21761 * t2258 + 5.0_f64 / 108.0_f64 * t56 * t10368 * t5825 * t2251 - 80.0_f64 / 27.0_f64 * t614 * t21736 + 20.0_f64 / 81.0_f64 * t614 * t21733 - 5.0_f64 / 108.0_f64 * t44 * t21732 * t2258 + 5.0_f64 / 162.0_f64 * t44 * t46065 * t5819 * t2251 + 5.0_f64 / 9.0_f64 * t44 * t4201 * t13312 - 40.0_f64 / 27.0_f64 * t614 * t21742 + 5.0_f64 / 9.0_f64 * t44 * t2275 * t18281 * t606 + 5.0_f64 / 18.0_f64 * t44 * t21741 * t2258 - 5.0_f64 / 108.0_f64 * t44 * t10355 * t5825 * t2251 - 80.0_f64 / 27.0_f64 * t1480 * t13325 - 40.0_f64 / 27.0_f64 * t1480 * t13328;
    t60987
}
