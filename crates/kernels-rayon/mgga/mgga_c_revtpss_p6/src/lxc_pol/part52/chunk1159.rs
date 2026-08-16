//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1159/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1159(t25081: f64, t8697: f64, t32782: f64, t571: f64, t2110: f64, t7337: f64, t2045: f64, t7541: f64, t1464: f64, t8720: f64, t2118: f64, t7318: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t122647 = t8697 * t25081;
    let t122710 = t571 * t32782;
    let t122712 = t2110 * t7337;
    let t122714 = t7541 * t2045;
    let t122720 = t8720 * t1464;
    let t122722 = t7318 * t2118;
    (t122647, t122710, t122712, t122714, t122720, t122722)
}
