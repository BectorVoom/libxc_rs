//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1040/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1040(t119868: f64, t2453: f64, t8464: f64, t817: f64, t8485: f64, t93341: f64, t119927: f64, t7063: f64, t119930: f64, t120043: f64, t31831: f64, t120004: f64, t25386: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120132 = t2453 * t8464 * t119868;
    let t120133 = 0.13386901839087538753e-4_f64 * t120132;
    let t120138 = t93341 * t8485 * t817;
    let t120140 = t7063 * t119927;
    let t120141 = t120140 * t119930;
    let t120149 = t31831 * t120043;
    let t120151 = t25386 * t120004;
    (t120133, t120138, t120140, t120141, t120149, t120151)
}
