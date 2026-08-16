//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1129/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1129(t39643: f64, t8476: f64, t119822: f64, t25386: f64, t2670: f64, t31831: f64, t119839: f64, t119968: f64, t31805: f64, t860: f64, t817: f64, t8485: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t119971 = t8476 * t39643;
    let t119982 = t25386 * t119822;
    let t119989 = t31831 * t2670;
    let t119991 = t119968 * t119839;
    let t119992 = 0.150583822711895824e-3_f64 * t119991;
    let t120000 = t31805 * t860;
    let t120002 = t120000 * t8485 * t817;
    (t119971, t119982, t119989, t119992, t120000, t120002)
}
