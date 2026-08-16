//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 530/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk530(t107: f64, t200: f64, t202: f64, t205: f64, t262: f64, t705: f64, t716: f64, t198: f64, t206: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2357 = 1.0_f64 / t107;
    let t2375 = 1.0_f64 / t200;
    let t2382 = 1.0_f64 / t202;
    let t2393 = t205 * t262;
    let t2398 = t705 * t716;
    let t2403 = t198 * t206;
    (t2357, t2375, t2382, t2393, t2398, t2403)
}
