//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3870/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3870(t1413: f64, t21969: f64, t547: f64, t807: f64, t13789: f64, t13790: f64, t1410: f64, t3829: f64, t46627: f64, t46828: f64, t46831: f64, t46833: f64, t46837: f64, t46840: f64, t46859: f64, t46861: f64, t48756: f64, t5671: f64, t6836: f64, t73837: f64, t828: f64) -> f64 {
    let t74402 = t807 * t547 * t1413 * t21969;
    let t74418 = 0.36143185997963725432e-4_f64 * t46828 + 0.57165357490759649296e-4_f64 * t74402 - 0.68598428988911579156e-2_f64 * t5671 * t13789 * t13790 * t73837 - t46831 + 0.90702367218671976884e-1_f64 * t48756 + 0.16264433699083676444e-3_f64 * t46833 - 0.2032800112371413129e-4_f64 * t46837 + t46840 - 0.40164115440237189888e-6_f64 * t46859 + 0.13552000749142754193e-3_f64 * t46861 + 0.18007087609589289528e0_f64 * t1410 * t46627 * t828 * t6836 * t3829;
    t74418
}
