//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 335/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk335(t1015: f64, t606: f64, t1012: f64, t225: f64, t989: f64, t366: f64, t994: f64) -> (f64, f64, f64, f64, f64) {
    let t1016 = t1015 * t606;
    let t1017 = t1012 * t1016;
    let t1020 = t989 * t225;
    let t1021 = t1020 * t366;
    let t1024 = t994 * t225;
    (t1016, t1017, t1020, t1021, t1024)
}
