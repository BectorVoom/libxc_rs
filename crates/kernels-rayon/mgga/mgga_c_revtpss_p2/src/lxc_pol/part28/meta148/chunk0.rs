//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 798/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk798(t1016: f64, t140: f64, t1011: f64, t1015: f64, t2258: f64, t1012: f64, t271: f64, t905: f64) -> (f64, f64, f64, f64, f64) {
    let t3244 = t140 * t1016;
    let t3245 = t1011 * t3244;
    let t3247 = t1015 * t2258;
    let t3248 = t1012 * t3247;
    let t3252 = 1.0_f64 / t271 / t905;
    (t3244, t3245, t3247, t3248, t3252)
}
