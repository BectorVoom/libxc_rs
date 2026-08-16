//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2634/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2634(t231: f64, t2782: f64, t2783: f64, t6041: f64, t836: f64, t61756: f64, t2797: f64, t136: f64, t2457: f64, t2710: f64, t10535: f64, t5978: f64) -> (f64, f64, f64, f64) {
    let t62693 = t2782 * t2783 * t6041 * t836 * t231;
    let t62695 = t61756 * t231;
    let t62697 = t2782 * t2797 * t62695;
    let t62716 = t2710 * t6041 * t136 * t2457;
    let t62723 = t10535 * t5978 * t136 * t2457;
    (t62693, t62697, t62716, t62723)
}
