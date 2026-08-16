//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2124/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2124(t28002: f64, t686: f64, t72: f64, t25895: f64, t5722: f64, t94748: f64, t1444: f64, t5675: f64, t98067: f64, t27968: f64, t3920: f64, t1445: f64, t27985: f64, t689: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t98356 = t28002 * t72 * t686;
    let t98358 = 0.28912093960683998208e-1_f64 * t25895 * t98356;
    let t98360 = 0.19514881078765566038e-1_f64 * t94748 * t5722;
    let t98362 = t5675 * t1444;
    let t98368 = 0.28912093960683998208e-1_f64 * t25895 * t98067;
    let t98372 = t27968 * t3920;
    let t98376 = 0.10975748638225852664e-1_f64 * t689 * t27985 * t1445;
    (t98356, t98358, t98360, t98362, t98368, t98372, t98376)
}
