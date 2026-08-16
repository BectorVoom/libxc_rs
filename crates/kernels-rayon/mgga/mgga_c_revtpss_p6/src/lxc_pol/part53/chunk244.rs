//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 244/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk244(t1043: f64, t373: f64, t357: f64, t73: f64, t1042: f64, t362: f64, t39: f64, t40: f64, t361: f64, t351: f64, t127: f64, t371: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1044 = t373 * t1043;
    let t1045 = t73 * t357;
    let t1046 = t1044 * t1045;
    let t1047 = t1042 * t1046;
    let t1050 = t362 * t39;
    let t1052 = 1.0_f64 / t40 / t1050;
    let t1053 = t361 * t1052;
    let t1054 = t351 * t1053;
    let t1058 = t371 * t127 * t373;
    (t1045, t1046, t1047, t1052, t1053, t1054, t1058)
}
