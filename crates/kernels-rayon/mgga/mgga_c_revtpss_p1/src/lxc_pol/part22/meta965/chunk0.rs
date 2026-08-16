//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3228/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3228(t40121: f64, t50058: f64, t40127: f64, t40132: f64, t18263: f64, t2414: f64, t40207: f64, t6002: f64, t40139: f64, t50084: f64, t14353: f64, t14365: f64, t18871: f64, t2403: f64, t40131: f64, t40137: f64, t4433: f64, t4541: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t61214 = 0.20779030926817756511e3_f64 * t40121;
    let t61215 = 16.0_f64 * t50058;
    let t61219 = 0.24415263074675393405e-3_f64 * t40127;
    let t61220 = 0.11696447245269292414e1_f64 * t40132;
    let t61222 = 4.0_f64 * t18263 * t2414;
    let t61224 = 12.0_f64 * t40207 * t6002;
    let t61225 = 8.0_f64 * t40139;
    let t61229 = 8.0_f64 * t50084;
    let t61230 = 24.0_f64 * t14353 * t4433 * t4541 + 12.0_f64 * t14365 * t18871 * t2403 - t40131 - t40137 + t61214 + t61215 + t61219 - t61220 + t61222 + t61224 + t61225 + t61229;
    (t61214, t61215, t61219, t61220, t61222, t61224, t61225, t61229, t61230)
}
