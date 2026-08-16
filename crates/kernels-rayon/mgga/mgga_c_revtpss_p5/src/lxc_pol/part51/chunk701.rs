//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 701/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk701(t225: f64, t7759: f64, t1568: f64, t1955: f64, t1579: f64, t1949: f64, t7071: f64, t1558: f64, t231: f64, t7076: f64, t233: f64, t1957: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7760 = t7759 * t225;
    let t7766 = t1955 * t1568;
    let t7769 = t1949 * t1579;
    let t7770 = t7071 * t7769;
    let t7774 = t1949 * t1558 * t231;
    let t7775 = t7076 * t7774;
    let t7778 = t233 * t7759;
    let t7779 = t1957 * t7778;
    (t7760, t7766, t7769, t7770, t7774, t7775, t7778, t7779)
}
