//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 167/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk167(t460: f64, t495: f64, t198: f64, t336: f64, t424: f64, t452: f64, t454: f64, t265: f64) -> (f64, f64, f64) {
    let t498 = 1.0_f64 + 0.65854491829355115987e0_f64 * t460 * t495;
    let t499 = f64::ln(t498);
    let t502 = t198 * t336 * t499 - t424 + t452 + t454;
    let t503 = t265 < t502;
    let t504 = piecewise3(t503, t502, t265);
    (t498, t504, t502)
}
