//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 861/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk861(t2847: f64, t4571: f64, t6094: f64, t6098: f64, t6102: f64, t291: f64, t1610: f64, t4590: f64, t1609: f64, t935: f64, t2874: f64, t1600: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6104 = t2847 + 0.11872222222222222222e-1_f64 * t4571 - 0.11872222222222222222e-1_f64 * t6094 + 0.35616666666666666666e-1_f64 * t6098 - 0.17808333333333333333e-1_f64 * t6102;
    let t6106 = 0.621814e-1_f64 * t6104 * t291;
    let t6108 = 2.0_f64 * t4590 * t1610;
    let t6109 = t1609 * t1609;
    let t6110 = t6109 * t935;
    let t6112 = 2.0_f64 * t2874 * t6110;
    let t6113 = t1600 * t1600;
    (t6104, t6106, t6108, t6109, t6110, t6112, t6113)
}
