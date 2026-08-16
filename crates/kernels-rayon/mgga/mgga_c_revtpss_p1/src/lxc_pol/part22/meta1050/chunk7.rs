//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3702/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3702(t44865: f64, t68253: f64, t68255: f64, t68257: f64, t68262: f64, t68267: f64, t68271: f64, t68275: f64, t68277: f64, t68282: f64, t68287: f64, t68292: f64) -> f64 {
    let t70158 = 0.59266666666666666668e-1_f64 * t68253 + 0.65851851851851851853e-2_f64 * t68255 - 0.43901234567901234569e-2_f64 * t68257 + t44865 - 0.10975308641975308642e-1_f64 * t68262 + 0.16462962962962962963e-1_f64 * t68267 + 0.35560000000000000001e0_f64 * t68271 + 0.59266666666666666668e-1_f64 * t68275 - 0.19755555555555555556e-1_f64 * t68277 - 0.19755555555555555556e-1_f64 * t68282 - 0.9877777777777777778e-2_f64 * t68287 - 0.59266666666666666668e-1_f64 * t68292;
    t70158
}
