//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1193/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1193(t93012: f64, t2453: f64, t2783: f64, t64: f64, t7030: f64, t9784: f64, t2482: f64, t25260: f64, t27: f64, t596: f64, t7036: f64, t2681: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t93013 = 0.22589491248727328397e-6_f64 * t93012;
    let t93015 = t2453 * t2783 * t64;
    let t93020 = t9784 * t7030;
    let t93021 = 0.14450132032386466905e-2_f64 * t93020;
    let t93025 = t2482 * t25260 * t27;
    let t93034 = t2482 * t7036 * t596;
    let t93048 = t820 * t7036 * t2681;
    (t93013, t93015, t93021, t93025, t93034, t93048)
}
