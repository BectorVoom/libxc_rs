//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1814/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1814(t73350: f64, t48225: f64, t85895: f64, t48227: f64, t73360: f64, t48243: f64, t39483: f64, t39520: f64, t39528: f64, t39531: f64, t39747: f64, t46972: f64, t46980: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t91958 = 6.0_f64 * t73350;
    let t91959 = 48.0_f64 * t48225;
    let t91960 = 0.23392894490538584828e1_f64 * t85895;
    let t91961 = 240.0_f64 * t48227;
    let t91962 = 48.0_f64 * t73360;
    let t91963 = 4.0_f64 * t48243;
    let t91964 = t91958 - t46972 - t39483 - t91959 + t39520 - t91960 + t91961 - t39528 - t91962 + t39531 + t91963 + t46980 + t39747;
    (t91958, t91959, t91960, t91961, t91962, t91963, t91964)
}
