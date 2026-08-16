//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 857/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk857(t1035: f64, t8515: f64, t1983: f64, t378: f64, t7150: f64, t8521: f64, t995: f64, t342: f64, t7135: f64, t1071: f64, t3140: f64, t1078: f64, t1982: f64) -> (f64, f64, f64, f64, f64) {
    let t25604 = t8515 * t1035;
    let t25605 = t1983 * t25604;
    let t25610 = t7150 * t378;
    let t25611 = t25610 * t8521;
    let t25629 = t995 * t8521;
    let t25634 = t342 * t7135;
    let t25638 = t1071 * t3140;
    let t25640 = t1982 * t25638 * t1078;
    (t25605, t25611, t25629, t25634, t25640)
}
