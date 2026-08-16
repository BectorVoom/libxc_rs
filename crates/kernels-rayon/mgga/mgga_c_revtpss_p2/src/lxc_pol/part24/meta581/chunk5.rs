//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1810/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1810(t30: f64, t33: f64, t6785: f64, t5824: f64, t1344: f64, t21944: f64, t22670: f64, t3874: f64, t46310: f64, t5574: f64, t87125: f64, t6792: f64, t6416: f64, t1348: f64, t21956: f64, t22783: f64, t3881: f64, t46328: f64, t5582: f64, t89780: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t91797 = t6785 * t6785;
    let t91802 = t5824 * t5824;
    let t91810 = piecewise3(t31, 0.0_f64, -56.0_f64 / 81.0_f64 * t46310 * t91797 + 16.0_f64 / 9.0_f64 * t21944 * t5824 - 2.0_f64 / 3.0_f64 * t3874 * t91802 - 8.0_f64 / 9.0_f64 * t5574 * t22670 + 2.0_f64 / 3.0_f64 * t1344 * t87125);
    let t91811 = t6792 * t6792;
    let t91816 = t6416 * t6416;
    let t91824 = piecewise3(t34, 0.0_f64, -56.0_f64 / 81.0_f64 * t46328 * t91811 + 16.0_f64 / 9.0_f64 * t21956 * t6416 - 2.0_f64 / 3.0_f64 * t3881 * t91816 - 8.0_f64 / 9.0_f64 * t5582 * t22783 + 2.0_f64 / 3.0_f64 * t1348 * t89780);
    (t91797, t91802, t91810, t91811, t91816, t91824)
}
