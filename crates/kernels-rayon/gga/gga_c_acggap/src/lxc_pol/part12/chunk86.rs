//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 86/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk86(t244: f64, t40: f64, t67: f64, t62: f64, t205: f64, t207: f64, t211: f64, t216: f64, t70: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t245 = t40 * t244;
    let t249 = t67 * t67;
    let t250 = 1.0_f64 / t249;
    let t251 = t62 * t250;
    let t256 = -0.1176575e1_f64 * t205 - 0.516475e0_f64 * t207 - 0.2103875e0_f64 * t211 - 0.104195e0_f64 * t216;
    let t257 = 1.0_f64 / t70;
    (t245, t249, t250, t251, t256, t257)
}
