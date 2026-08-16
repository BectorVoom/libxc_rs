//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 781/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk781(t1045: f64, t999: f64, t1043: f64, t3155: f64, t12131: f64, t357: f64, t1448: f64, t1868: f64, t197: f64, t531: f64, t2013: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19620 = t1045 * t999;
    let t19634 = t3155 * t1043;
    let t19639 = t12131 * t357;
    let t22496 = t1868 * t1448;
    let t25081 = t197 * t531;
    let t25082 = t2013 * t25081;
    (t19620, t19634, t19639, t22496, t25081, t25082)
}
