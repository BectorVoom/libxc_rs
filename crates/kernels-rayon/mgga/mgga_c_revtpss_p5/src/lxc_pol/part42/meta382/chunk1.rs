//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1262/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1262(t19330: f64, t934: f64, t2924: f64, t11466: f64, t11507: f64, t19294: f64, t19297: f64, t19300: f64, t19304: f64, t19307: f64, t19311: f64, t19315: f64, t19317: f64, t19320: f64, t19323: f64, t19326: f64, t19329: f64, t2987: f64, t3012: f64) -> (f64, f64) {
    let t19331 = t19330 * t934;
    let t19333 = 0.16081979498692535067e2_f64 * t2924 * t19331;
    let t19334 = -0.23392894490538584828e1_f64 * t2987 * t19294 - 0.10389515463408878255e3_f64 * t11466 * t19297 - 0.11696447245269292414e1_f64 * t2987 * t19300 + 0.17315859105681463759e2_f64 * t3012 * t19304 + 0.34631718211362927518e2_f64 * t3012 * t19307 + 0.10254018858216406658e4_f64 * t11507 * t19311 + t19315 - t19317 - t19320 + t19323 + t19326 + t19329 - t19333;
    (t19333, t19334)
}
