//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 703/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk703(t532: f64, t7311: f64, t1450: f64, t2014: f64, t1448: f64, t4147: f64) -> (f64, f64, f64, f64) {
    let t7312 = t532 * t7311;
    let t7313 = t7312 * t1450;
    let t7314 = t2014 * t7313;
    let t7315 = t4147 * t1448;
    (t7312, t7313, t7314, t7315)
}
