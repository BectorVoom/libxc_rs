//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 817/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk817(t2828: f64, t886: f64, t2770: f64, t2435: f64, t2445: f64, t2441: f64, t9303: f64, t10115: f64, t258: f64, t2453: f64, t2464: f64, t2438: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10494 = t886 * t2828;
    let t10495 = t2770 * t10494;
    let t10498 = t2435 * t2445;
    let t10501 = 0.26019841438354088051e-2_f64 * t9303 * t2441;
    let t10503 = 0.11044544084478153697e-3_f64 * t10115 * t258;
    let t10504 = t2453 * t2464;
    let t10505 = t2438 * t886;
    (t10495, t10498, t10501, t10503, t10504, t10505)
}
