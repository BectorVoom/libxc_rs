//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1190/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1190(t126101: f64, t126153: f64, t126205: f64, t126241: f64, t126290: f64, t126333: f64, t126367: f64, t126408: f64, t892: f64, t198: f64, t205: f64, t8489: f64) -> (f64, f64, f64) {
    let t126411 = t126101 + t126153 + t126205 + t126241 + t126290 + t126333 + t126367 + t126408;
    let t126412 = t126411 * t892;
    let t126422 = t198 * t205 * t8489;
    (t126411, t126412, t126422)
}
