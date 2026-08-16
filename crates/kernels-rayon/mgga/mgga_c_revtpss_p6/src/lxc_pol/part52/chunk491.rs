//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 491/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk491(t225: f64, t3555: f64, t480: f64, t3566: f64, t1236: f64, t127: f64, t371: f64, t1235: f64, t221: f64, t462: f64, t696: f64, t461: f64) -> (f64, f64, f64, f64, f64) {
    let t3666 = t3555 * t225;
    let t3667 = t3666 * t480;
    let t3670 = t3566 * t225;
    let t3678 = t371 * t127 * t1236;
    let t3679 = t1235 * t3678;
    let t3682 = t221 * t696 * t462;
    let t3684 = t461 * t3682 / 432.0_f64;
    (t3666, t3667, t3670, t3679, t3684)
}
