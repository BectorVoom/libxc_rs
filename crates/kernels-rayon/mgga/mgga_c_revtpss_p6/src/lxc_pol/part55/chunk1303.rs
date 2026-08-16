//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1303/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1303(t129099: f64, t129103: f64, t129107: f64, t129109: f64, t129111: f64, t1918: f64, t2115: f64, t2170: f64, t28975: f64, t28981: f64, t29480: f64, t33328: f64, t34011: f64, t34014: f64, t5802: f64, t8616: f64, t8905: f64) -> f64 {
    let t131170 = 3.0_f64 * t1918 * t33328 + 3.0_f64 * t2115 * t29480 + 6.0_f64 * t2170 * t28975 + 6.0_f64 * t2170 * t28981 + 6.0_f64 * t5802 * t8905 + t129099 + t129103 + t129107 + t129109 + t129111 + t34011 + t34014 + t8616;
    t131170
}
