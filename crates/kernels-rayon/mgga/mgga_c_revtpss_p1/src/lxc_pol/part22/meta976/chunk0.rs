//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3284/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3284(t50873: f64, t40172: f64, t14330: f64, t18575: f64, t2258: f64, t14370: f64, t18259: f64, t18562: f64, t2626: f64, t18576: f64, t50895: f64, t5819: f64, t606: f64, t749: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t62269 = 16.0_f64 * t50873;
    let t62270 = 0.20508037716432813316e4_f64 * t40172;
    let t62273 = 24.0_f64 * t14330 * t18575 * t2258;
    let t62274 = t18259 * t14370;
    let t62275 = 48.0_f64 * t62274;
    let t62276 = t18562 * t2626;
    let t62277 = 0.11696447245269292414e1_f64 * t62276;
    let t62279 = 48.0_f64 * t50895 * t18576;
    let t62282 = t14330 * t749 * t5819 * t606;
    (t62269, t62270, t62273, t62275, t62277, t62279, t62282)
}
