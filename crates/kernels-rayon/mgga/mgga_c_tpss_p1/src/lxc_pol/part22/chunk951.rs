//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 951/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk951(t3054: f64, t3073: f64, t219: f64, t3111: f64, t1137: f64, t73: f64, t8549: f64, t9615: f64, t8548: f64, t9080: f64, t9619: f64, t3126: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9721 = t3054 * t3073;
    let t9730 = t3111 * t219;
    let t9737 = t1137 * t1137;
    let t9738 = 1.0_f64 / t9737;
    let t9739 = t73 * t9738;
    let t9748 = t8549 * t9615;
    let t9749 = t8548 * t9748;
    let t9751 = t9080 * t9619;
    let t9759 = t3126 * t3073;
    (t9721, t9730, t9739, t9749, t9751, t9759)
}
