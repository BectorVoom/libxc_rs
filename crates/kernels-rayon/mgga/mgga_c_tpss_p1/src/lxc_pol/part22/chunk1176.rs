//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1176/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1176(t13215: f64, t630: f64, t13154: f64, t13157: f64, t13159: f64, t13161: f64, t13165: f64, t13168: f64, t69: f64, t7587: f64, t7588: f64, t7590: f64, t7592: f64) -> f64 {
    let t13216 = t630 * t13215;
    let t13219 = -t7587 - 22.0_f64 / 9.0_f64 * t7588 - 2.0_f64 / 3.0_f64 * t7590 + t7592 / 3.0_f64 - 11.0_f64 / 9.0_f64 * t13154 - t13157 + t13159 - 3.0_f64 / 4.0_f64 * t69 * t13161 + t69 * t13165 / 2.0_f64 + t69 * t13168 / 4.0_f64 - t69 * t13216 / 8.0_f64;
    t13219
}
