//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1554/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1554(t1261: f64, t24643: f64, t3172: f64, t24770: f64, t3153: f64, t17569: f64, t20783: f64, t1222: f64, t140: f64, t24816: f64, t24820: f64, t12915: f64, t247: f64, t24713: f64, t5384: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t82827 = t1261 * t3172 * t24643;
    let t82859 = t24770 * t3153;
    let t82932 = t17569 * t20783;
    let t82980 = t1222 * t140 * t24816;
    let t82983 = t1222 * t140 * t24820;
    let t83014 = t5384 * t247 * t12915 * t24713;
    (t82827, t82859, t82932, t82980, t82983, t83014)
}
