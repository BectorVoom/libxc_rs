//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2915/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2915(t23492: f64, t698: f64, t23471: f64, t141: f64, t77501: f64, t930: f64, t18987: f64, t4606: f64, t15118: f64, t6120: f64, t18950: f64, t4614: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t77663 = t698 * t23492;
    let t77667 = t698 * t23471;
    let t77670 = t141 * t930 * t77501;
    let t77672 = t18987 * t4606;
    let t77674 = t15118 * t6120;
    let t77676 = t4614 * t18950;
    (t77663, t77667, t77670, t77672, t77674, t77676)
}
