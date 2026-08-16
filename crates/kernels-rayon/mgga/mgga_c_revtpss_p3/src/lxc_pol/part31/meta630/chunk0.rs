//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2084/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2084(t25569: f64, t4817: f64, t1659: f64, t25576: f64, t27489: f64, t3111: f64, t11940: f64, t7131: f64, t16158: f64, t7132: f64, t100007: f64, t16094: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t100097 = 0.3811023832717309953e-3_f64 * t25569 * t4817;
    let t100114 = t1659 * t25576;
    let t100117 = t27489 * t3111;
    let t100121 = t11940 * t7131;
    let t100132 = 0.3811023832717309953e-3_f64 * t7132 * t16158;
    let t100135 = t16094 * t100007;
    (t100097, t100114, t100117, t100121, t100132, t100135)
}
