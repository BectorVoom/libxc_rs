//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2133/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2133(t1937: f64, t98487: f64, t27123: f64, t6993: f64, t25803: f64, t7898: f64, t2033: f64, t47672: f64, t1907: f64, t4144: f64, t28196: f64, t27833: f64, t7313: f64) -> (f64, f64, f64, f64, f64) {
    let t98489 = 4.0_f64 * t98487 * t1937;
    let t98491 = 4.0_f64 * t27123 * t6993;
    let t98494 = t7898 * t25803;
    let t98495 = t2033 * t47672;
    let t98496 = t1907 * t4144;
    let t98499 = 6.0_f64 * t28196 * t98495 * t98496;
    let t98501 = 2.0_f64 * t27833 * t7313;
    (t98489, t98491, t98494, t98499, t98501)
}
