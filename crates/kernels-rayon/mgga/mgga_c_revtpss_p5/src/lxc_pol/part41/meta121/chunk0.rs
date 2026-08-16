//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 600/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk600(t1043: f64, t73: f64, t357: f64, t905: f64, t606: f64, t1052: f64, t369: f64, t361: f64, t351: f64, t1065: f64, t126: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3093 = t1043 * t73;
    let t3094 = t357 * t905;
    let t3095 = t3094 * t606;
    let t3104 = t1052 * t369;
    let t3105 = t361 * t3104;
    let t3106 = t351 * t3105;
    let t3109 = t126 * t1065;
    (t3093, t3094, t3095, t3105, t3106, t3109)
}
