//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 227/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk227(t378: f64, t994: f64, t225: f64, t385: f64, t902: f64, t908: f64) -> (f64, f64, f64, f64) {
    let t995 = t994 * t378;
    let t996 = t225 * t385;
    let t997 = 0.14816666666666666667e-1_f64 * t902;
    let t999 = -t997 - 0.14816666666666666667e-1_f64 * t908;
    (t995, t996, t997, t999)
}
