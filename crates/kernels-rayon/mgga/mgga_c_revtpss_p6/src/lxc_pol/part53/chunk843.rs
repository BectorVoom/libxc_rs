//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 843/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk843(t2470: f64, t7059: f64, t7064: f64, t1949: f64, t785: f64, t780: f64, t2439: f64, t212: f64, t7048: f64, t689: f64, t7014: f64, t887: f64) -> (f64, f64, f64, f64, f64) {
    let t25331 = t7059 * t2470;
    let t25333 = 0.17135234354032049604e-1_f64 * t7064 * t25331;
    let t25334 = t785 * t1949;
    let t25335 = t25334 * t780;
    let t25337 = 0.65049603595885220126e-3_f64 * t2439 * t25335;
    let t25338 = t212 * t7048;
    let t25339 = t25338 * t780;
    let t25340 = t689 * t25339;
    let t25352 = t7014 * t887;
    (t25331, t25333, t25337, t25340, t25352)
}
