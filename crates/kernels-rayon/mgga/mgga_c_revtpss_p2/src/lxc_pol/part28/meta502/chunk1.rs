//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1892/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1892(t26123: f64, t670: f64, t572: f64, t2371: f64, t7330: f64, t117: f64, t25832: f64, t1461: f64, t2040: f64, t26106: f64, t26115: f64, t26117: f64, t26119: f64, t26122: f64, t4162: f64, t4165: f64, t573: f64, t7324: f64) -> (f64, f64, f64, f64) {
    let t26124 = t26123 * t670;
    let t26126 = 12.0_f64 * t572 * t26124;
    let t26127 = t7330 * t2371;
    let t26129 = 6.0_f64 * t572 * t26127;
    let t26130 = t117 * t25832;
    let t26132 = 3.0_f64 * t572 * t26130;
    let t26133 = 6.0_f64 * t1461 * t7324 + 6.0_f64 * t2040 * t4162 + 3.0_f64 * t2040 * t4165 + t26106 * t573 + t26115 + t26117 + t26119 + t26122 + t26126 + t26129 + t26132;
    (t26124, t26127, t26130, t26133)
}
