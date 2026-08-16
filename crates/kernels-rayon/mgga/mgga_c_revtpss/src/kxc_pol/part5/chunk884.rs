//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 884/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk884(t1699: f64, t1102: f64, t198: f64, t3336: f64, t336: f64, t6106: f64, t6108: f64, t6112: f64, t6144: f64, t6147: f64, t6213: f64, t6215: f64, t6217: f64, t6221: f64, t6225: f64, t6229: f64, t6396: f64) -> (f64, f64) {
    let t6400 = t1699 * t1699;
    let t6404 = t1102 * t198 * t336 * t6396 - t198 * t3336 * t336 * t6400 - t6106 + t6108 - t6112 + t6144 + t6147 + t6213 + t6215 - t6217 + t6221 - t6225 - t6229;
    (t6400, t6404)
}
