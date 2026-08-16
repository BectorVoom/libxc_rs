//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 125/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk125(t344: f64, t56: f64, t404: f64, t221: f64, t65: f64, t225: f64, t460: f64) -> (f64, f64, f64, f64) {
    let t461 = t56 * t344;
    let t462 = 1.0_f64 / t404;
    let t464 = t221 * t65 * t462;
    let t467 = t460 * t225;
    (t461, t462, t464, t467)
}
