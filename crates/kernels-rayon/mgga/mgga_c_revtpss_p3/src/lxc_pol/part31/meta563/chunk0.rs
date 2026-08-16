//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1974/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1974(t231: f64, t61756: f64, t1544: f64, t2411: f64, t22461: f64, t4147: f64, t6861: f64, t9994: f64, t1398: f64, t221: f64, t22274: f64, t22279: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t62695 = t61756 * t231;
    let t63185 = t2411 * t1544;
    let t73407 = t22461 * t4147;
    let t73820 = t6861 * t9994;
    let t73842 = t6861 * t1398;
    let t74419 = t221 * t22274;
    let t74423 = t221 * t22279;
    (t62695, t63185, t73407, t73820, t73842, t74419, t74423)
}
