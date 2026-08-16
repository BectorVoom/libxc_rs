//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1998/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1998(t2434: f64, t837: f64, t25377: f64, t25431: f64, t251: f64, t25304: f64, t25374: f64, t10505: f64, t93172: f64, t2453: f64, t25398: f64, t10506: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t93182 = t2434 * t837;
    let t93183 = t25377 * t93182;
    let t93184 = t25431 * t93183;
    let t93189 = t25304 * t251;
    let t93190 = t93189 * t25374;
    let t93191 = t93172 * t10505;
    let t93192 = t93190 * t93191;
    let t93194 = t2453 * t25398;
    let t93195 = t93194 * t10506;
    (t93183, t93184, t93189, t93190, t93191, t93192, t93194, t93195)
}
