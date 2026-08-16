//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 828/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk828(t2441: f64, t9303: f64, t10115: f64, t258: f64, t2453: f64, t2464: f64, t251: f64, t4503: f64, t786: f64, t2797: f64, t760: f64, t9323: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10501 = 0.26019841438354088051e-2_f64 * t9303 * t2441;
    let t10503 = 0.11044544084478153697e-3_f64 * t10115 * t258;
    let t10504 = t2453 * t2464;
    let t10529 = t4503 * t251;
    let t10530 = t786 * t10529;
    let t10535 = t2453 * t2797;
    let t10552 = 0.51947577317044391277e2_f64 * t760 * t9323;
    (t10501, t10503, t10504, t10530, t10535, t10552)
}
