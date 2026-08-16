//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1905/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1905(t2150: f64, t29109: f64, t473: f64, t2142: f64, t5245: f64, t7637: f64, t1243: f64, t8190: f64, t1248: f64, t1287: f64, t1811: f64, t3140: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29111 = t2150 * t473 * t29109;
    let t29118 = t2142 * t5245;
    let t29119 = t7637 * t29118;
    let t29122 = t1243 * t8190;
    let t29124 = t29122 * t1248 * t1287;
    let t29127 = t1811 * t3140;
    (t29111, t29118, t29119, t29122, t29124, t29127)
}
