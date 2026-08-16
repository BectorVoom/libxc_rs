//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1075/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1075(t3145: f64, t334: f64, t368: f64, t3153: f64, t73: f64, t246: f64, t676: f64, t1046: f64, t1041: f64, t1038: f64, t3229: f64, t1036: f64) -> (f64, f64, f64, f64, f64) {
    let t11243 = 1.0_f64 / t3145 / t368 / t334;
    let t11249 = t3153 * t73;
    let t11262 = t246 * t676;
    let t11263 = t11262 * t1046;
    let t11264 = t1041 * t11263;
    let t11266 = t3229 * t1038;
    let t11267 = t1036 * t11266;
    (t11243, t11249, t11262, t11264, t11267)
}
