//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 892/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk892(t2014: f64, t28187: f64, t7315: f64, t7934: f64, t7235: f64, t7901: f64, t7937: f64, t2013: f64, t8995: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28188 = t2014 * t28187;
    let t28189 = t7934 * t7315;
    let t28190 = t2014 * t28189;
    let t28192 = 3.0_f64 * t7235 * t7901;
    let t28193 = t7235 * t7937;
    let t28196 = t2013 * t8995;
    (t28188, t28189, t28190, t28192, t28193, t28196)
}
