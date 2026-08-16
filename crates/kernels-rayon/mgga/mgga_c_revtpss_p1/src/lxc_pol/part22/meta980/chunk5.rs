//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3307/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3307(t2482: f64, t2801: f64, t5977: f64, t879: f64, t10073: f64, t18750: f64, t231: f64, t2782: f64, t2783: f64, t6041: f64, t836: f64, t61756: f64) -> (f64, f64, f64, f64) {
    let t62682 = t2482 * t879 * t5977 * t2801;
    let t62684 = t10073 * t18750;
    let t62693 = t2782 * t2783 * t6041 * t836 * t231;
    let t62695 = t61756 * t231;
    (t62682, t62684, t62693, t62695)
}
