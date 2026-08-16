//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1401/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1401(t98: f64, t106: f64, t143: f64, t2580: f64, t130: f64, t2566: f64, t700: f64, t2584: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9163 = t98 * t98;
    let t9232 = t106 * t106;
    let t9273 = 1.0_f64 / t2580 / t143;
    let t9274 = t130 * t9273;
    let t9275 = t2566 * t700;
    let t9276 = t9275 * t2584;
    let t9278 = 0.96491876992155210402e2_f64 * t9274 * t9276;
    (t9163, t9232, t9273, t9274, t9275, t9276, t9278)
}
