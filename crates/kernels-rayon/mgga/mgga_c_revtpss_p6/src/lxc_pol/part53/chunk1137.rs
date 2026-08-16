//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1137/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1137(t32121: f64, t7898: f64, t32103: f64, t7732: f64, t2322: f64, t33591: f64, t25082: f64, t27153: f64, t36970: f64, t32322: f64, t7937: f64, t13648: f64, t2014: f64, t8595: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t125514 = 3.0_f64 * t7898 * t32121;
    let t125515 = t7732 * t32103;
    let t125517 = t2322 * t33591;
    let t125521 = 3.0_f64 * t25082 * t36970 * t27153;
    let t125522 = t32322 * t7937;
    let t125525 = t2014 * t8595 * t13648;
    (t125514, t125515, t125517, t125521, t125522, t125525)
}
