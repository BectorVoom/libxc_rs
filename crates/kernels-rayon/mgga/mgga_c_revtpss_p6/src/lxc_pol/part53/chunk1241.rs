//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1241/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1241(t2170: f64, t28268: f64, t28277: f64, t28265: f64, t28280: f64, t127496: f64, t127498: f64, t127500: f64, t127503: f64, t127507: f64, t1918: f64, t32897: f64, t34014: f64, t8616: f64) -> f64 {
    let t129570 = t2170 * t28268;
    let t129572 = t2170 * t28277;
    let t129574 = t2170 * t28265;
    let t129577 = t2170 * t28280;
    let t129580 = 3.0_f64 * t1918 * t32897 + 3.0_f64 * t127496 + 3.0_f64 * t127498 + 6.0_f64 * t127500 + t127503 + t127507 + 6.0_f64 * t129570 + 6.0_f64 * t129572 + 6.0_f64 * t129574 + 3.0_f64 * t129577 + t34014 + t8616;
    t129580
}
