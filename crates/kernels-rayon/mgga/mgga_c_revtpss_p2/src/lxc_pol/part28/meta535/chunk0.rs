//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1978/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1978(t5: f64, t28115: f64, t28157: f64, t117: f64, t7239: f64, t7898: f64, t197: f64, t530: f64, t2013: f64) -> (f64, f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t28159 = piecewise3(t8, 0.0_f64, t28115 + t28157);
    let t28160 = t28159 * t117;
    let t28165 = 3.0_f64 * t7898 * t7239;
    let t28166 = t197 * t530;
    let t28167 = t2013 * t28166;
    (t28159, t28160, t28165, t28166, t28167)
}
