//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2256/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2256(t60221: f64, t6957: f64, t13269: f64, t607: f64, t13272: f64, t25105: f64, t10309: f64, t28126: f64, t1493: f64, t2248: f64, t77: f64, t1928: f64, t25099: f64, t25102: f64, t25106: f64, t25110: f64, t25157: f64, t25159: f64, t28081: f64, t28127: f64, t28133: f64, t6960: f64, t6963: f64, t7706: f64, t7720: f64, t92666: f64, t92699: f64) -> f64 {
    let t101320 = t60221 * t6957;
    let t101323 = t13269 * t607;
    let t101326 = t13272 * t25105;
    let t101333 = t10309 * t28126;
    let t101337 = t77 * t1493 * t2248;
    let t101340 = 5.0_f64 / 3.0_f64 * t28127 * t25110 + 2.0_f64 / 3.0_f64 * t6963 * t28081 + 5.0_f64 / 3.0_f64 * t25099 * t28133 + 2.0_f64 / 3.0_f64 * t25102 * t7720 + 5.0_f64 / 3.0_f64 * t25106 * t28133 + 5.0_f64 / 3.0_f64 * t101320 * t6960 + 2.0_f64 / 3.0_f64 * t101323 * t1928 + 5.0_f64 / 3.0_f64 * t101326 * t6960 - 5.0_f64 / 3.0_f64 * t92666 * t7706 + 5.0_f64 / 6.0_f64 * t92699 * t7706 - 5.0_f64 * t101333 * t25159 - 5.0_f64 * t25157 * t101337;
    t101340
}
