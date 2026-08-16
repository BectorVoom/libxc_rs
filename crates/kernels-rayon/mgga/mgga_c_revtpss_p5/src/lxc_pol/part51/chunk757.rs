//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 757/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk757(t532: f64, t8594: f64, t1450: f64, t2014: f64, t2033: f64) -> (f64, f64, f64, f64) {
    let t8595 = t532 * t8594;
    let t8596 = t8595 * t1450;
    let t8597 = t2014 * t8596;
    let t8598 = t2033 * t2033;
    (t8595, t8596, t8597, t8598)
}
