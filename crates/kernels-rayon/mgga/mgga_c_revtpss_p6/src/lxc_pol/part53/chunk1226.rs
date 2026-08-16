//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1226/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1226(t127313: f64, t127314: f64, t127318: f64, t127324: f64, t127326: f64, t127328: f64, t127330: f64, t127332: f64, t127335: f64, t32107: f64, t32109: f64, t32112: f64, t8463: f64) -> f64 {
    let t129426 = t127313 + t127314 + t127318 - 2.0_f64 * t127324 - 2.0_f64 * t127326 - 2.0_f64 * t127328 - 2.0_f64 * t127330 + t127332 - t8463 + t127335 - t32107 - t32109 - t32112;
    t129426
}
