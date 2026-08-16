//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1327/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1327(t3145: f64, t334: f64, t368: f64, t3153: f64, t73: f64, t246: f64, t676: f64, t1046: f64, t1041: f64, t3140: f64, t989: f64, t3149: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11243 = 1.0_f64 / t3145 / t368 / t334;
    let t11249 = t3153 * t73;
    let t11262 = t246 * t676;
    let t11263 = t11262 * t1046;
    let t11264 = t1041 * t11263;
    let t11273 = t989 * t3140;
    let t11274 = t11273 * t3149;
    (t11243, t11249, t11262, t11264, t11273, t11274)
}
