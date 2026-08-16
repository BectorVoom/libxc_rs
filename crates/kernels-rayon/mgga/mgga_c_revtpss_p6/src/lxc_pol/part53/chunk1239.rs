//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1239/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1239(t7334: f64, t8245: f64, t7331: f64, t127465: f64, t127468: f64, t127472: f64, t127475: f64, t127480: f64, t127481: f64, t127483: f64, t129523: f64, t573: f64, t5802: f64, t8771: f64, param_d: f64) -> f64 {
    let t129541 = t8245 * t7334;
    let t129543 = t8245 * t7331;
    let t129552 = t129523 * t573 * param_d + 6.0_f64 * t5802 * t8771 + t127465 + t127468 + t127472 + 6.0_f64 * t127475 + t127480 + 6.0_f64 * t127481 + 6.0_f64 * t127483 + 3.0_f64 * t129541 + 6.0_f64 * t129543;
    t129552
}
