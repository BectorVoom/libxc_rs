//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 825/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk825(t9736: f64, t9738: f64, t1369: f64, t2699: f64, t1372: f64, t3943: f64, t794: f64, t3946: f64, t1412: f64, t159: f64, t216: f64, t124: f64, t800: f64, t9400: f64) -> (f64, f64, f64, f64, f64) {
    let t9739 = t9736 * t9738;
    let t9741 = t2699 * t1369;
    let t9742 = t9741 * t1372;
    let t9744 = t794 * t3943;
    let t9745 = t9744 * t3946;
    let t9747 = t159 * t1412;
    let t9748 = t216 * t9747;
    let t9750 = t800 * t124 * t9400;
    (t9739, t9742, t9745, t9748, t9750)
}
