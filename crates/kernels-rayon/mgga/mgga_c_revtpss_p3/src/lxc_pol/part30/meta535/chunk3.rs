//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1960/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1960(t29118: f64, t7637: f64, t1243: f64, t8190: f64, t1248: f64, t1287: f64, t1811: f64, t3140: f64, t1276: f64, t2148: f64, t5412: f64, t1032: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29119 = t7637 * t29118;
    let t29122 = t1243 * t8190;
    let t29124 = t29122 * t1248 * t1287;
    let t29127 = t1811 * t3140;
    let t29129 = t2148 * t29127 * t1276;
    let t29132 = t2148 * t5412;
    let t29135 = t1811 * t1032;
    (t29119, t29122, t29124, t29129, t29132, t29135)
}
