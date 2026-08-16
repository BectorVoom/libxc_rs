//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1301/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1301(t127455: f64, t127459: f64, t127462: f64, t129034: f64, t129039: f64, t129045: f64, t129048: f64, t129055: f64, t129057: f64, t129065: f64, t1461: f64, t2170: f64, t28978: f64, t34838: f64, t7554: f64, t8245: f64) -> f64 {
    let t131155 = 3.0_f64 * t1461 * t34838 + 6.0_f64 * t2170 * t28978 + 6.0_f64 * t7554 * t8245 + t127455 + t127459 + t127462 + t129034 + t129039 + t129045 + t129048 + t129055 + t129057 + t129065;
    t131155
}
