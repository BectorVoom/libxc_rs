//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1102/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1102(t119927: f64, t7063: f64, t119930: f64, t120043: f64, t31831: f64, t120004: f64, t25386: f64, t120006: f64, t2453: f64, t31798: f64, t119974: f64, t25304: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t120140 = t7063 * t119927;
    let t120141 = t120140 * t119930;
    let t120149 = t31831 * t120043;
    let t120151 = t25386 * t120004;
    let t120152 = t120151 * t120006;
    let t120154 = t2453 * t31798;
    let t120156 = 0.95199562775170587692e-3_f64 * t120154 * t119974;
    let t120157 = t25304 * t31798;
    (t120140, t120141, t120149, t120151, t120152, t120156, t120157)
}
