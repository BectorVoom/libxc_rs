//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2261/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2261(t13517: f64, t196: f64, t197: f64, t2035: f64, t28196: f64, t28197: f64, t75365: f64, t94976: f64, t1513: f64, t94975: f64, t28036: f64, t94978: f64) -> (f64, f64, f64, f64, f64) {
    let t101435 = t13517 * t196 * t197;
    let t101436 = t101435 * t2035;
    let t101439 = 4.0_f64 * t28196 * t28197 * t75365;
    let t101448 = 22.0_f64 / 9.0_f64 * t94976;
    let t101451 = t94975 * t1513;
    let t101453 = t94978 * t28036;
    (t101436, t101439, t101448, t101451, t101453)
}
