//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 989/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk989(t2441: f64, t9303: f64, t10115: f64, t258: f64, t2453: f64, t2464: f64, t2438: f64, t886: f64, t138: f64, t2434: f64, t123: f64, t2465: f64) -> (f64, f64, f64, f64, f64) {
    let t10501 = 0.26019841438354088051e-2_f64 * t9303 * t2441;
    let t10503 = 0.11044544084478153697e-3_f64 * t10115 * t258;
    let t10504 = t2453 * t2464;
    let t10505 = t2438 * t886;
    let t10506 = t138 * t10505;
    let t10507 = t10504 * t10506;
    let t10509 = t2434 * t886;
    let t10510 = t123 * t10509;
    let t10511 = t2465 * t10510;
    (t10501, t10503, t10504, t10507, t10511)
}
