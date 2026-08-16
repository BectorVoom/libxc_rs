//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1534/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1534(t11921: f64, t23964: f64, t247: f64, t4837: f64, t11246: f64, t23833: f64, t3172: f64, t1063: f64, t23851: f64, t1011: f64, t140: f64, t23873: f64) -> (f64, f64, f64, f64) {
    let t79564 = t4837 * t247 * t11921 * t23964;
    let t79575 = t11246 * t3172 * t23833;
    let t79580 = t1063 * t3172 * t23851;
    let t79638 = t1011 * t140 * t23873;
    (t79564, t79575, t79580, t79638)
}
