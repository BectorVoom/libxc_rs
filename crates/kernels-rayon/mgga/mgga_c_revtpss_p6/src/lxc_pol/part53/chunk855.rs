//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 855/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk855(t1078: f64, t1982: f64, t25638: f64, t1976: f64, t3057: f64, t989: f64, t1035: f64, t7135: f64, t3046: f64, t994: f64, t11199: f64, t1981: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25640 = t1982 * t25638 * t1078;
    let t25651 = t3057 * t1976;
    let t25658 = t989 * t1976;
    let t25681 = t1035 * t7135;
    let t25692 = t3046 * t1976;
    let t25695 = t994 * t7135;
    let t25698 = t1981 * t11199;
    (t25640, t25651, t25658, t25681, t25692, t25695, t25698)
}
