//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2232/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2232(t104752: f64, t104756: f64, t104758: f64, t104762: f64, t104768: f64, t104770: f64, t1797: f64, t26873: f64, t29010: f64, t3591: f64, t3606: f64, t3613: f64, t3714: f64, t5287: f64, t97120: f64, t97171: f64, t97177: f64) -> f64 {
    let t104772 = -0.3811023832717309953e-3_f64 * t97171 + 0.42874018118069736972e-3_f64 * t97120 * t1797 + 0.85748036236139473944e-3_f64 * t26873 * t5287 + 0.57165357490759649296e-3_f64 * t104752 * t3714 + t104756 - 0.45732285992607719436e-2_f64 * t104758 * t3606 + 0.22866142996303859718e-2_f64 * t104762 * t3613 + 0.42874018118069736972e-3_f64 * t29010 * t3591 + t104768 - t104770 + t97177 / 648.0_f64;
    t104772
}
