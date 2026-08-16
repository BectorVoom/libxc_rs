//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1162/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1162(t1209: f64, t29135: f64, t2142: f64, t5219: f64, t3801: f64, t8220: f64, t1479: f64, t60: f64, t2122: f64, t28150: f64, t13272: f64, t7565: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29275 = t1209 * t29135;
    let t29304 = t5219 * t2142;
    let t29317 = t8220 * t3801;
    let t29355 = t1479 * t60;
    let t29380 = t2122 * t28150;
    let t29388 = t13272 * t7565;
    (t29275, t29304, t29317, t29355, t29380, t29388)
}
