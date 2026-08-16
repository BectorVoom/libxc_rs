//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1034/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1034(t2438: f64, t886: f64, t138: f64, t10504: f64, t2434: f64, t123: f64, t2465: f64, t215: f64, t231: f64, t268: f64, t836: f64, t2798: f64) -> (f64, f64, f64) {
    let t10505 = t2438 * t886;
    let t10506 = t138 * t10505;
    let t10507 = t10504 * t10506;
    let t10509 = t2434 * t886;
    let t10510 = t123 * t10509;
    let t10511 = t2465 * t10510;
    let t10518 = t268 * t215 * t836 * t231;
    let t10519 = t2798 * t10518;
    (t10507, t10511, t10519)
}
