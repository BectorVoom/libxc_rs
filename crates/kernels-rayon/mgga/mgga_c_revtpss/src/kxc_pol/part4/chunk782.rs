//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 782/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk782(t1448: f64, t1450: f64, t565: f64, t1343: f64, t1353: f64, t198: f64, t3871: f64, t3873: f64, t3889: f64, t4025: f64, t4027: f64, t4031: f64, t4033: f64, t4035: f64, t4037: f64, t4040: f64, t4042: f64, t4135: f64, t4139: f64, t532: f64) -> (f64, f64, f64, f64, f64) {
    let t4140 = t1448 * t1450;
    let t4144 = t1448 * t1448;
    let t4146 = t565 * t565;
    let t4147 = 1.0_f64 / t4146;
    let t4150 = t1450 * t198 * t4135 * t532 - t198 * t4144 * t4147 * t532 + 3.0_f64 * t1343 * t198 * t3889 + 6.0_f64 * t1353 * t4139 * t4140 + t3871 + t3873 + t4025 + t4027 + t4031 - t4033 - t4035 - t4037 - t4040 + t4042;
    (t4140, t4144, t4146, t4147, t4150)
}
